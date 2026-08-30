use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct BenchmarkResult {
    name: String,
    min_ns: u64,
    avg_ns: u64,
    p50_ns: u64,
    p95_ns: u64,
    p99_ns: u64,
    max_ns: u64,
    stddev_ns: f64,
}

fn analyze_latencies(name: &str, mut lats: Vec<u64>) -> BenchmarkResult {
    lats.sort_unstable();
    let n = lats.len();
    let sum: u64 = lats.iter().sum();
    let avg = sum / n as u64;
    let min = *lats.first().unwrap();
    let max = *lats.last().unwrap();
    let p50 = lats[n * 50 / 100];
    let p95 = lats[n * 95 / 100];
    let p99 = lats[n * 99 / 100];

    let mut variance = 0.0;
    for &v in &lats {
        let diff = v as f64 - avg as f64;
        variance += diff * diff;
    }
    let stddev = (variance / n as f64).sqrt();

    BenchmarkResult {
        name: name.to_string(),
        min_ns: min,
        avg_ns: avg,
        p50_ns: p50,
        p95_ns: p95,
        p99_ns: p99,
        max_ns: max,
        stddev_ns: stddev,
    }
}

fn print_result(r: &BenchmarkResult) {
    println!("--- [{}] ---", r.name);
    println!("  Min:    {:.3} us", r.min_ns as f64 / 1000.0);
    println!("  Avg:    {:.3} us", r.avg_ns as f64 / 1000.0);
    println!("  P50:    {:.3} us", r.p50_ns as f64 / 1000.0);
    println!("  P95:    {:.3} us", r.p95_ns as f64 / 1000.0);
    println!("  P99:    {:.3} us", r.p99_ns as f64 / 1000.0);
    println!("  Max:    {:.3} us", r.max_ns as f64 / 1000.0);
    println!("  StdDev: {:.3} us", r.stddev_ns / 1000.0);
}

// Scenario 1: Protocol Parsing & State Machine
#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Idle,
    Running,
    Alarm,
    Stopped,
}

struct Packet {
    magic: [u8; 2],
    seq: u16,
    cmd: u32,
    payload_len: u32,
    payload: [u8; 64],
    checksum: u32,
}

fn bench_protocol(results: &mut Vec<BenchmarkResult>) {
    let mut lats = Vec::with_capacity(1000);
    let mut current_state = State::Idle;

    let mut pkt = Packet {
        magic: [0x55, 0xAA],
        seq: 1,
        cmd: 0x02,
        payload_len: 32,
        payload: [0u8; 64],
        checksum: 0x12345678,
    };
    for i in 0..32 {
        pkt.payload[i] = i as u8;
    }

    for _ in 0..1000 {
        let t1 = Instant::now();

        let mut calc_sum: u32 = 0;
        for j in 0..pkt.payload_len as usize {
            calc_sum += pkt.payload[j] as u32;
        }

        if pkt.magic == [0x55, 0xAA] {
            current_state = match current_state {
                State::Idle => State::Running,
                State::Running => {
                    if calc_sum > 1000 {
                        State::Alarm
                    } else {
                        State::Running
                    }
                }
                _ => State::Idle,
            };
        }

        let elapsed = t1.elapsed();
        lats.push(elapsed.as_nanos() as u64);
    }
    results.push(analyze_latencies("Rust: Protocol & State Machine", lats));
}

// Scenario 2: Dynamic Allocation & Math
fn bench_alloc_math(results: &mut Vec<BenchmarkResult>) {
    let mut lats = Vec::with_capacity(500);

    for _ in 0..500 {
        let t1 = Instant::now();
        let mut v: Vec<f64> = Vec::with_capacity(100);
        for j in 0..100 {
            let val = ((j as f64) * 0.01).sin() * ((j as f64) + 1.0).sqrt();
            v.push(val);
        }
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let _total: f64 = v.iter().sum();
        let elapsed = t1.elapsed();
        lats.push(elapsed.as_nanos() as u64);
    }
    results.push(analyze_latencies("Rust: Dynamic Alloc & Math", lats));
}

// Scenario 3: Periodic Loop Jitter (10ms)
fn bench_periodic_jitter(results: &mut Vec<BenchmarkResult>) {
    let mut lats = Vec::with_capacity(50);
    for _ in 0..50 {
        let t1 = Instant::now();
        thread::sleep(Duration::from_millis(10));
        let elapsed = t1.elapsed();
        lats.push(elapsed.as_nanos() as u64);
    }
    results.push(analyze_latencies("Rust: 10ms Periodic Loop Jitter", lats));
}

// Scenario 4: Multi-threaded Work Queue
fn bench_work_queue(results: &mut Vec<BenchmarkResult>) {
    let mut lats = Vec::with_capacity(200);

    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let done = Arc::new(AtomicBool::new(false));
    let processed = Arc::new(AtomicU32::new(0));

    let mut workers = Vec::new();
    for _ in 0..2 {
        let q = Arc::clone(&queue);
        let d = Arc::clone(&done);
        let p = Arc::clone(&processed);
        workers.push(thread::spawn(move || {
            while !d.load(Ordering::Relaxed) || !q.lock().unwrap().is_empty() {
                let item = {
                    let mut lock = q.lock().unwrap();
                    lock.pop_front()
                };
                if item.is_some() {
                    p.fetch_add(1, Ordering::Relaxed);
                } else {
                    thread::yield_now();
                }
            }
        }));
    }

    for i in 0..200 {
        let t1 = Instant::now();
        {
            let mut lock = queue.lock().unwrap();
            for j in 0..5 {
                lock.push_back(i * 5 + j);
            }
        }
        let elapsed = t1.elapsed();
        lats.push(elapsed.as_nanos() as u64);
    }

    done.store(true, Ordering::Relaxed);
    for w in workers {
        w.join().unwrap();
    }
    results.push(analyze_latencies("Rust: Work Queue Synchronization", lats));
}

fn print_memory_usage() {
    println!("--- [Rust Memory Usage] ---");
    if let Ok(file) = File::open("/proc/self/status") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("VmPeak:") || line.starts_with("VmRSS:") {
                println!("  {}", line);
            }
        }
    }
}

fn main() {
    println!("========================================");
    println!(" PowerPC P1022 (e500v2) Rust Benchmark  ");
    println!("========================================");

    let mut results = Vec::new();
    bench_protocol(&mut results);
    bench_alloc_math(&mut results);
    bench_periodic_jitter(&mut results);
    bench_work_queue(&mut results);

    for r in &results {
        print_result(r);
    }
    print_memory_usage();

    println!("========================================");
    println!(" Rust Benchmark Completed Successfully  ");
    println!("========================================");
}
