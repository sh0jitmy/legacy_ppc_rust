#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "=================================================="
echo " Building PowerPC P1022 Static Binaries (muslspe) "
echo " Target: powerpc-unknown-linux-muslspe (Static)   "
echo "=================================================="

cd "${WORKSPACE_ROOT}"

# Ensure sysroot objects are in rustlib
RUST_SYSROOT=$(rustc +nightly --print sysroot)
DEST="${RUST_SYSROOT}/lib/rustlib/powerpc-unknown-linux-muslspe/lib"
mkdir -p "${DEST}"
if [ -d "${WORKSPACE_ROOT}/target/musl-sysroot/lib" ]; then
    cp -f "${WORKSPACE_ROOT}/target/musl-sysroot/lib/"* "${DEST}/" 2>/dev/null || true
fi

# Build all test crates + benchmarks with static musl
cargo +nightly build \
    --package test_01_hello \
    --package test_02_heap_alloc \
    --package test_03_spe_float \
    --package test_04_threads_sync \
    --package test_05_timer_latency \
    --package test_06_networking \
    --package rust_benchmark \
    --target powerpc-unknown-linux-muslspe \
    -Z build-std=std,panic_abort \
    --release

mkdir -p target/binaries_musl

TESTS=(
    "test_01_hello"
    "test_02_heap_alloc"
    "test_03_spe_float"
    "test_04_threads_sync"
    "test_05_timer_latency"
    "test_06_networking"
    "rust_benchmark"
)

for bin in "${TESTS[@]}"; do
    SRC="target/powerpc-unknown-linux-muslspe/release/${bin}"
    if [ -f "${SRC}" ]; then
        cp "${SRC}" "target/binaries_musl/${bin}"
        echo "  [OK] ${bin}"
    fi
done

echo "=================================================="
echo " Static muslspe Binaries Built into target/binaries_musl/"
echo "=================================================="
