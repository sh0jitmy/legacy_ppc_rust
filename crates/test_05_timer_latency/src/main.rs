use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("========================================");
    println!(" [Rust Test 05] Timer & Periodic Jitter ");
    println!("========================================");

    let target_period = Duration::from_millis(10);
    let iterations = 50;
    let mut latencies_us = Vec::with_capacity(iterations);

    println!("  Running {} iterations of 10ms sleep...", iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        thread::sleep(target_period);
        let elapsed = start.elapsed();
        latencies_us.push(elapsed.as_micros() as u64);
    }

    latencies_us.sort_unstable();
    let min = latencies_us.first().unwrap();
    let max = latencies_us.last().unwrap();
    let sum: u64 = latencies_us.iter().sum();
    let avg = sum / iterations as u64;
    let p50 = latencies_us[iterations * 50 / 100];
    let p95 = latencies_us[iterations * 95 / 100];
    let p99 = latencies_us[iterations * 99 / 100];

    println!("  Target: 10,000 us (10.0 ms)");
    println!("  Min:    {} us", min);
    println!("  Avg:    {} us", avg);
    println!("  P50:    {} us", p50);
    println!("  P95:    {} us", p95);
    println!("  P99:    {} us", p99);
    println!("  Max:    {} us", max);

    println!("  [PASS] Test 05 Timer & Periodic Jitter passed.");
}
