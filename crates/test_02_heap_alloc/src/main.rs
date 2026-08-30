use std::collections::HashMap;

fn main() {
    println!("========================================");
    println!(" [Rust Test 02] Heap & Allocator Test   ");
    println!("========================================");

    // 1. Vector dynamic allocation & resizing
    let mut v = Vec::with_capacity(10);
    for i in 0..10000 {
        v.push(i);
    }
    assert_eq!(v.len(), 10000);
    assert_eq!(v[9999], 9999);
    println!("  [OK] Vec 10,000 items allocated and verified.");

    // 2. Box allocation
    let boxed = Box::new(42u64);
    assert_eq!(*boxed, 42);
    println!("  [OK] Box allocation verified.");

    // 3. String manipulation
    let mut s = String::from("Legacy PPC ");
    s.push_str("Rust Environment");
    assert_eq!(s, "Legacy PPC Rust Environment");
    println!("  [OK] String manipulation verified.");

    // 4. HashMap hashing & insertion
    let mut map = HashMap::new();
    for i in 0..500 {
        map.insert(format!("key_{}", i), i * 2);
    }
    assert_eq!(map.get("key_250"), Some(&500));
    println!("  [OK] HashMap 500 entries verified.");

    println!("  [PASS] Test 02 Heap & Allocator passed.");
}
