use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("========================================");
    println!(" [Rust Test 04] Threads & Sync Test     ");
    println!("========================================");

    // 1. AtomicU32 & AtomicBool
    let counter = Arc::new(AtomicU32::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();

    for id in 0..4 {
        let cnt = Arc::clone(&counter);
        let flg = Arc::clone(&flag);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                cnt.fetch_add(1, Ordering::SeqCst);
            }
            if id == 3 {
                flg.store(true, Ordering::Release);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 4000);
    assert!(flag.load(Ordering::Acquire));
    println!("  [OK] 4 threads atomic incremented to 4000.");

    // 2. Arc<Mutex<Vec<u32>>>
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for id in 0..4 {
        let d = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            for i in 0..250 {
                let mut guard = d.lock().unwrap();
                guard.push(id * 1000 + i);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let final_data = data.lock().unwrap();
    assert_eq!(final_data.len(), 1000);
    println!("  [OK] Mutex protected Vec reached 1000 elements.");

    println!("  [PASS] Test 04 Threads & Sync passed.");
}
