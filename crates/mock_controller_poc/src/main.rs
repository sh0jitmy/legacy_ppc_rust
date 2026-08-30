use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

extern "C" {
    fn c_actuator_output(channel: u32, target_val: f64) -> i32;
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ControllerState {
    Init,
    Standby,
    ControlActive,
    Failsafe,
}

#[derive(Serialize, Deserialize, Debug)]
struct TelemetryMessage {
    timestamp_ms: u64,
    state: String,
    sensor_value: f64,
    actuator_cmd: f64,
    cycle_latency_us: u64,
}

struct ControlLoop {
    state: ControllerState,
    cycle_count: u32,
    target_value: f64,
}

impl ControlLoop {
    fn new() -> Self {
        Self {
            state: ControllerState::Init,
            cycle_count: 0,
            target_value: 0.0,
        }
    }

    fn step(&mut self, sensor_input: f64) -> (f64, u64) {
        let t0 = Instant::now();
        self.cycle_count += 1;

        // State Machine Transition
        self.state = match self.state {
            ControllerState::Init => ControllerState::Standby,
            ControllerState::Standby => {
                if self.cycle_count >= 2 {
                    ControllerState::ControlActive
                } else {
                    ControllerState::Standby
                }
            }
            ControllerState::ControlActive => {
                if sensor_input > 100.0 {
                    ControllerState::Failsafe
                } else {
                    ControllerState::ControlActive
                }
            }
            ControllerState::Failsafe => ControllerState::Failsafe,
        };

        // PID / Control algorithm calculation
        let err = self.target_value - sensor_input;
        let mut actuator_out = 0.0;
        if self.state == ControllerState::ControlActive {
            // SPE float computation
            actuator_out = (err * 1.25) + ((self.cycle_count as f64) * 0.05).sin();
            unsafe {
                let _res = c_actuator_output(1, actuator_out);
            }
        }

        let elapsed_us = t0.elapsed().as_micros() as u64;
        (actuator_out, elapsed_us)
    }
}

fn main() {
    println!("========================================");
    println!(" [Rust PoC] Mock Embedded Controller    ");
    println!(" Target: PowerPC P1022 (e500v2)         ");
    println!(" Period: 20ms Control Cycle             ");
    println!("========================================");

    let mut controller = ControlLoop::new();
    controller.target_value = 50.0;

    let telemetry_queue = Arc::new(Mutex::new(VecDeque::new()));
    let running = Arc::new(AtomicBool::new(true));

    // Telemetry consumer thread (Serializes JSON telemetry)
    let q_clone = Arc::clone(&telemetry_queue);
    let r_clone = Arc::clone(&running);
    let telemetry_handle = thread::spawn(move || {
        let mut count = 0;
        while r_clone.load(Ordering::Relaxed) || !q_clone.lock().unwrap().is_empty() {
            let msg = {
                let mut q = q_clone.lock().unwrap();
                q.pop_front()
            };
            if let Some(t) = msg {
                let json = serde_json::to_string(&t).unwrap();
                if count % 10 == 0 {
                    println!("  [Telemetry JSON] {}", json);
                }
                count += 1;
            } else {
                thread::sleep(Duration::from_millis(5));
            }
        }
        println!("  Telemetry worker finished (processed {} messages).", count);
    });

    // Main Control Loop (20ms periodic cycle, 30 cycles)
    println!("  Starting 20ms periodic control loop...");
    let mut latencies = Vec::new();

    for cycle in 0..30 {
        let loop_start = Instant::now();
        let sensor_sim = 45.0 + ((cycle as f64) * 0.2).sin() * 4.0;
        let (actuator, calc_us) = controller.step(sensor_sim);
        latencies.push(calc_us);

        // Enqueue telemetry
        let tele = TelemetryMessage {
            timestamp_ms: (cycle * 20) as u64,
            state: format!("{:?}", controller.state),
            sensor_value: sensor_sim,
            actuator_cmd: actuator,
            cycle_latency_us: calc_us,
        };
        telemetry_queue.lock().unwrap().push_back(tele);

        let cycle_duration = loop_start.elapsed();
        if cycle_duration < Duration::from_millis(20) {
            thread::sleep(Duration::from_millis(20) - cycle_duration);
        }
    }

    running.store(false, Ordering::Relaxed);
    telemetry_handle.join().unwrap();

    latencies.sort_unstable();
    let avg = latencies.iter().sum::<u64>() / latencies.len() as u64;
    let max = *latencies.last().unwrap();
    println!("  Control step computation latency: Avg = {} us, Max = {} us", avg, max);
    println!("  [PASS] Mock Embedded Controller PoC completed successfully.");
}
