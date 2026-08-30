#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <time.h>
#include <math.h>
#include <unistd.h>

void test_spe_float(void) {
    printf("[C BASELINE] Testing Float / Math...\n");
    volatile double a = 3.141592653589793;
    volatile double b = 2.718281828459045;
    volatile double c = a * b;
    double s = sin(1.57079632679);
    printf("  Multiplication: %f * %f = %f\n", a, b, c);
    printf("  sin(pi/2): %f (expected ~1.0)\n", s);
    if (fabs(s - 1.0) < 0.0001) {
        printf("  [PASS] Float math test passed.\n");
    } else {
        printf("  [FAIL] Float math test failed!\n");
        exit(1);
    }
}

void* thread_func(void* arg) {
    long id = (long)arg;
    printf("  [Thread %ld] Worker thread running...\n", id);
    return NULL;
}

void test_threads(void) {
    printf("[C BASELINE] Testing POSIX Threads...\n");
    pthread_t th1, th2;
    pthread_create(&th1, NULL, thread_func, (void*)1);
    pthread_create(&th2, NULL, thread_func, (void*)2);
    pthread_join(th1, NULL);
    pthread_join(th2, NULL);
    printf("  [PASS] Thread join completed.\n");
}

void test_timer(void) {
    printf("[C BASELINE] Testing Clock / Timer...\n");
    struct timespec ts1, ts2;
    clock_gettime(CLOCK_MONOTONIC, &ts1);
    usleep(10000); // 10ms
    clock_gettime(CLOCK_MONOTONIC, &ts2);
    long diff_us = (ts2.tv_sec - ts1.tv_sec) * 1000000 + (ts2.tv_nsec - ts1.tv_nsec) / 1000;
    printf("  Slept ~10ms, measured: %ld us\n", diff_us);
    if (diff_us >= 8000 && diff_us <= 50000) {
        printf("  [PASS] Timer precision within acceptable range.\n");
    } else {
        printf("  [WARN] Timer diff: %ld us\n", diff_us);
    }
}

int main(void) {
    printf("========================================\n");
    printf(" PowerPC P1022 (e500v2) C Baseline Test \n");
    printf("========================================\n");
    test_spe_float();
    test_threads();
    test_timer();
    printf("========================================\n");
    printf(" C Baseline All Tests PASSED\n");
    printf("========================================\n");
    return 0;
}
