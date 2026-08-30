#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include <math.h>
#include <pthread.h>
#include <unistd.h>
#include <string.h>

#define NUM_RUNS 1000

typedef struct {
    char name[64];
    uint64_t min_ns;
    uint64_t avg_ns;
    uint64_t p50_ns;
    uint64_t p95_ns;
    uint64_t p99_ns;
    uint64_t max_ns;
    double stddev_ns;
} BenchmarkResult;

static inline uint64_t get_time_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}

int cmp_u64(const void* a, const void* b) {
    uint64_t va = *(const uint64_t*)a;
    uint64_t vb = *(const uint64_t*)b;
    return (va > vb) - (va < vb);
}

int cmp_double(const void* a, const void* b) {
    double va = *(const double*)a;
    double vb = *(const double*)b;
    return (va > vb) - (va < vb);
}

BenchmarkResult analyze_latencies(const char* name, uint64_t* lats, size_t n) {
    qsort(lats, n, sizeof(uint64_t), cmp_u64);
    uint64_t sum = 0;
    for (size_t i = 0; i < n; ++i) sum += lats[i];
    uint64_t avg = sum / n;
    uint64_t min = lats[0];
    uint64_t max = lats[n - 1];
    uint64_t p50 = lats[n * 50 / 100];
    uint64_t p95 = lats[n * 95 / 100];
    uint64_t p99 = lats[n * 99 / 100];

    double variance = 0.0;
    for (size_t i = 0; i < n; ++i) {
        double diff = (double)lats[i] - (double)avg;
        variance += diff * diff;
    }
    double stddev = sqrt(variance / n);

    BenchmarkResult r;
    strncpy(r.name, name, sizeof(r.name) - 1);
    r.min_ns = min;
    r.avg_ns = avg;
    r.p50_ns = p50;
    r.p95_ns = p95;
    r.p99_ns = p99;
    r.max_ns = max;
    r.stddev_ns = stddev;
    return r;
}

void print_result(const BenchmarkResult* r) {
    printf("--- [%s] ---\n", r->name);
    printf("  Min:    %.3f us\n", r->min_ns / 1000.0);
    printf("  Avg:    %.3f us\n", r->avg_ns / 1000.0);
    printf("  P50:    %.3f us\n", r->p50_ns / 1000.0);
    printf("  P95:    %.3f us\n", r->p95_ns / 1000.0);
    printf("  P99:    %.3f us\n", r->p99_ns / 1000.0);
    printf("  Max:    %.3f us\n", r->max_ns / 1000.0);
    printf("  StdDev: %.3f us\n", r->stddev_ns / 1000.0);
}

// Scenario 1: Protocol Parsing & State Machine
typedef enum { STATE_IDLE, STATE_RUNNING, STATE_ALARM, STATE_STOPPED } State;

typedef struct {
    uint8_t magic[2];
    uint16_t seq;
    uint32_t cmd;
    uint32_t payload_len;
    uint8_t payload[64];
    uint32_t checksum;
} Packet;

void bench_protocol(BenchmarkResult* out_r) {
    uint64_t lats[1000];
    State current_state = STATE_IDLE;

    Packet pkt;
    pkt.magic[0] = 0x55; pkt.magic[1] = 0xAA;
    pkt.seq = 1;
    pkt.cmd = 0x02;
    pkt.payload_len = 32;
    for (int i = 0; i < 32; ++i) pkt.payload[i] = (uint8_t)i;
    pkt.checksum = 0x12345678;

    for (int i = 0; i < 1000; ++i) {
        uint64_t t1 = get_time_ns();
        
        uint32_t calc_sum = 0;
        for (uint32_t j = 0; j < pkt.payload_len; ++j) {
            calc_sum += pkt.payload[j];
        }
        if (pkt.magic[0] == 0x55 && pkt.magic[1] == 0xAA) {
            switch (current_state) {
                case STATE_IDLE: current_state = STATE_RUNNING; break;
                case STATE_RUNNING: current_state = (calc_sum > 1000) ? STATE_ALARM : STATE_RUNNING; break;
                default: current_state = STATE_IDLE; break;
            }
        }
        
        uint64_t t2 = get_time_ns();
        lats[i] = t2 - t1;
    }
    *out_r = analyze_latencies("CPP: Protocol & State Machine", lats, 1000);
}

// Scenario 2: Dynamic Allocation & Math
void bench_alloc_math(BenchmarkResult* out_r) {
    uint64_t lats[500];

    for (int i = 0; i < 500; ++i) {
        uint64_t t1 = get_time_ns();
        double* v = (double*)malloc(100 * sizeof(double));
        for (int j = 0; j < 100; ++j) {
            v[j] = sin((double)j * 0.01) * sqrt((double)j + 1.0);
        }
        qsort(v, 100, sizeof(double), cmp_double);
        double total = 0.0;
        for (int j = 0; j < 100; ++j) total += v[j];
        (void)total;
        free(v);
        uint64_t t2 = get_time_ns();
        lats[i] = t2 - t1;
    }
    *out_r = analyze_latencies("CPP: Dynamic Alloc & Math", lats, 500);
}

// Scenario 3: Periodic Loop Jitter (10ms)
void bench_periodic_jitter(BenchmarkResult* out_r) {
    uint64_t lats[50];
    for (int i = 0; i < 50; ++i) {
        uint64_t t1 = get_time_ns();
        usleep(10000); // 10ms
        uint64_t t2 = get_time_ns();
        lats[i] = t2 - t1;
    }
    *out_r = analyze_latencies("CPP: 10ms Periodic Loop Jitter", lats, 50);
}

// Scenario 4: Work Queue with Mutex
typedef struct {
    int items[1024];
    int head;
    int tail;
    int count;
    pthread_mutex_t mtx;
    int done;
    int processed;
} WorkQueue;

WorkQueue g_queue;

void* worker_thread(void* arg) {
    (void)arg;
    while (1) {
        int item = -1;
        pthread_mutex_lock(&g_queue.mtx);
        if (g_queue.count > 0) {
            item = g_queue.items[g_queue.head];
            g_queue.head = (g_queue.head + 1) % 1024;
            g_queue.count--;
            g_queue.processed++;
        } else if (g_queue.done) {
            pthread_mutex_unlock(&g_queue.mtx);
            break;
        }
        pthread_mutex_unlock(&g_queue.mtx);
        if (item < 0) {
            sched_yield();
        }
    }
    return NULL;
}

void bench_work_queue(BenchmarkResult* out_r) {
    uint64_t lats[200];
    memset(&g_queue, 0, sizeof(g_queue));
    pthread_mutex_init(&g_queue.mtx, NULL);

    pthread_t w1, w2;
    pthread_create(&w1, NULL, worker_thread, NULL);
    pthread_create(&w2, NULL, worker_thread, NULL);

    for (int i = 0; i < 200; ++i) {
        uint64_t t1 = get_time_ns();
        pthread_mutex_lock(&g_queue.mtx);
        for (int j = 0; j < 5; ++j) {
            g_queue.items[g_queue.tail] = i * 5 + j;
            g_queue.tail = (g_queue.tail + 1) % 1024;
            g_queue.count++;
        }
        pthread_mutex_unlock(&g_queue.mtx);
        uint64_t t2 = get_time_ns();
        lats[i] = t2 - t1;
    }

    pthread_mutex_lock(&g_queue.mtx);
    g_queue.done = 1;
    pthread_mutex_unlock(&g_queue.mtx);

    pthread_join(w1, NULL);
    pthread_join(w2, NULL);
    pthread_mutex_destroy(&g_queue.mtx);

    *out_r = analyze_latencies("CPP: Work Queue Synchronization", lats, 200);
}

void print_memory_usage(void) {
    FILE* fp = fopen("/proc/self/status", "r");
    if (!fp) return;
    char line[128];
    printf("--- [CPP Memory Usage] ---\n");
    while (fgets(line, sizeof(line), fp)) {
        if (strncmp(line, "VmPeak:", 7) == 0 || strncmp(line, "VmRSS:", 6) == 0) {
            printf("  %s", line);
        }
    }
    fclose(fp);
}

int main(void) {
    printf("========================================\n");
    printf(" PowerPC P1022 (e500v2) C/C++ Benchmark \n");
    printf("========================================\n");

    BenchmarkResult r1, r2, r3, r4;
    bench_protocol(&r1);
    bench_alloc_math(&r2);
    bench_periodic_jitter(&r3);
    bench_work_queue(&r4);

    print_result(&r1);
    print_result(&r2);
    print_result(&r3);
    print_result(&r4);
    print_memory_usage();

    printf("========================================\n");
    printf(" C/C++ Benchmark Completed Successfully \n");
    printf("========================================\n");
    return 0;
}
