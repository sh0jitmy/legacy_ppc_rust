fn main() {
    cc::Build::new()
        .file("c_src/legacy_driver.c")
        .flag("-mcpu=8548")
        .flag("-mabi=spe")
        .flag("-mspe")
        .compile("legacy_driver");
}
