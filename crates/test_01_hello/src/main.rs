fn main() {
    println!("========================================");
    println!(" [Rust Test 01] Hello World on PowerPC ");
    println!(" Target: powerpc-unknown-linux-gnuspe   ");
    println!(" Core:   e500v2 (P1022)                 ");
    println!("========================================");
    let target_arch = if cfg!(target_arch = "powerpc") { "powerpc" } else { "unknown" };
    let target_endian = if cfg!(target_endian = "big") { "big" } else { "little" };
    let pointer_width = usize::BITS;
    
    println!("  Architecture:  {}", target_arch);
    println!("  Endianness:    {}", target_endian);
    println!("  Pointer Width: {} bits", pointer_width);
    
    assert_eq!(target_arch, "powerpc");
    assert_eq!(target_endian, "big");
    assert_eq!(pointer_width, 32);
    
    println!("  [PASS] Test 01 Hello World passed.");
}
