fn main() {
    println!("========================================");
    println!(" [Rust Test 03] SPE Floating Point Test ");
    println!("========================================");

    // 1. Single Precision (f32)
    let f1: f32 = 1.5;
    let f2: f32 = 2.25;
    let f3 = f1 + f2;
    let f4 = f1 * f2;
    let f5 = f2 / f1;
    println!("  f32 addition:       1.5 + 2.25 = {}", f3);
    println!("  f32 multiplication: 1.5 * 2.25 = {}", f4);
    println!("  f32 division:       2.25 / 1.5 = {}", f5);
    assert!((f3 - 3.75).abs() < 1e-6);
    assert!((f4 - 3.375).abs() < 1e-6);
    assert!((f5 - 1.5).abs() < 1e-6);
    println!("  [OK] f32 basic operations verified.");

    // 2. Double Precision (f64)
    let d1: f64 = 3.141592653589793;
    let d2: f64 = 2.718281828459045;
    let d3 = d1 * d2;
    let d4 = d1.sqrt();
    let d5 = d1.sin();
    println!("  f64 mult:           pi * e = {}", d3);
    println!("  f64 sqrt:           sqrt(pi) = {}", d4);
    println!("  f64 sin:            sin(pi) = {}", d5);
    assert!((d3 - 8.539734222673566).abs() < 1e-12);
    assert!((d4 - 1.772453850905516).abs() < 1e-12);
    assert!(d5.abs() < 1e-12);
    println!("  [OK] f64 math operations verified.");

    // 3. Int to Float / Float to Int Casts
    let i: i32 = -12345;
    let fi = i as f64;
    let back_i = fi as i32;
    assert_eq!(i, back_i);
    println!("  [OK] Int <-> Float casting verified.");

    println!("  [PASS] Test 03 SPE Floating Point passed.");
}
