#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "=================================================="
echo " Building PowerPC P1022 (e500v2) Verification Suite"
echo " Target Triple: powerpc-unknown-linux-gnuspe"
echo "=================================================="

export CC=powerpc-linux-gnu-gcc
export CXX=powerpc-linux-gnu-g++
export AR=powerpc-linux-gnu-ar
export PKG_CONFIG_ALLOW_CROSS=1

# 1. Build C Baseline
echo "--> [1/4] Building C Baseline..."
make -C crates/c_baseline clean
make -C crates/c_baseline CC="${CC}"

# 2. Build C++ Benchmark
echo "--> [2/4] Building C++ Benchmark..."
make -C benches/cpp_benchmark clean
make -C benches/cpp_benchmark CXX="${CXX}"

# 3. Build Rust Workspace (Tests + PoC + Benchmark)
echo "--> [3/4] Building Rust Crates & Benchmarks with build-std..."
cargo +nightly build \
    --target powerpc-unknown-linux-gnuspe \
    -Z build-std=std,panic_abort \
    --release

echo "--> [4/4] Verifying all generated ELF binaries..."
mkdir -p target/binaries

cp crates/c_baseline/c_baseline target/binaries/c_baseline
cp benches/cpp_benchmark/cpp_benchmark target/binaries/cpp_benchmark

RUST_BINARIES=(
    "test_01_hello"
    "test_02_heap_alloc"
    "test_03_spe_float"
    "test_04_threads_sync"
    "test_05_timer_latency"
    "test_06_networking"
    "test_07_ffi"
    "test_08_https_rest"
    "mock_controller_poc"
    "rust_benchmark"
)

for bin in "${RUST_BINARIES[@]}"; do
    SRC="target/powerpc-unknown-linux-gnuspe/release/${bin}"
    if [ -f "${SRC}" ]; then
        cp "${SRC}" "target/binaries/${bin}"
        echo "  [OK] ${bin}"
    else
        echo "  [WARN] ${bin} not found at ${SRC}"
    fi
done

echo "=================================================="
echo " All Binaries Built Successfully into target/binaries/"
echo "=================================================="
