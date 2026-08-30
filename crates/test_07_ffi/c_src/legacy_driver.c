#include <stdint.h>
#include <stdio.h>

typedef struct {
    uint32_t channel_id;
    uint32_t raw_value;
    double calibrated_float;
    uint8_t status_flags;
} SensorData;

typedef int (*DataCallback)(const SensorData* data);

int c_legacy_init_hardware(uint32_t base_addr) {
    printf("  [C DRIVER] Initializing hardware at 0x%08X...\n", base_addr);
    return 0;
}

int c_legacy_process_sensor(SensorData* inout_data, DataCallback cb) {
    if (!inout_data) return -1;
    // Emulate calibration using SPE double float
    inout_data->calibrated_float = (double)inout_data->raw_value * 0.00125;
    inout_data->status_flags = 0x01; // Ready
    
    if (cb) {
        return cb(inout_data);
    }
    return 0;
}
