#[repr(C)]
#[derive(Debug)]
pub struct SensorData {
    pub channel_id: u32,
    pub raw_value: u32,
    pub calibrated_float: f64,
    pub status_flags: u8,
}

type DataCallback = extern "C" fn(data: *const SensorData) -> i32;

extern "C" {
    fn c_legacy_init_hardware(base_addr: u32) -> i32;
    fn c_legacy_process_sensor(inout_data: *mut SensorData, cb: Option<DataCallback>) -> i32;
}

extern "C" fn rust_callback(data: *const SensorData) -> i32 {
    unsafe {
        let d = &*data;
        println!("  [Rust Callback] Received from C: channel={}, raw={}, calibrated={:.4}, status=0x{:02X}",
            d.channel_id, d.raw_value, d.calibrated_float, d.status_flags);
        assert_eq!(d.channel_id, 1);
        assert_eq!(d.raw_value, 4000);
        assert!((d.calibrated_float - 5.0).abs() < 1e-4);
    }
    0
}

fn main() {
    println!("========================================");
    println!(" [Rust Test 07] C/C++ FFI Interop Test  ");
    println!("========================================");

    unsafe {
        let init_ret = c_legacy_init_hardware(0xFFE00000);
        assert_eq!(init_ret, 0);

        let mut sensor = SensorData {
            channel_id: 1,
            raw_value: 4000,
            calibrated_float: 0.0,
            status_flags: 0,
        };

        let ret = c_legacy_process_sensor(&mut sensor, Some(rust_callback));
        assert_eq!(ret, 0);
        assert_eq!(sensor.status_flags, 0x01);
    }

    println!("  [PASS] Test 07 C/C++ FFI passed.");
}
