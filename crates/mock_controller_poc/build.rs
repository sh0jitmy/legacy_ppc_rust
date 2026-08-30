fn main() {
    cc::Build::new()
        .file("c_src/actuator_driver.c")
        .flag("-mcpu=8548")
        .flag("-mabi=spe")
        .flag("-mspe")
        .compile("actuator_driver");
}
