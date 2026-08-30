#include <stdint.h>
#include <stdio.h>

int c_actuator_output(uint32_t channel, double target_val) {
    // SPE float calculation in C driver
    double converted = target_val * 1.05;
    (void)converted;
    return 0; // Success
}
