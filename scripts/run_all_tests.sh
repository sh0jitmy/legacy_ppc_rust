#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "=================================================="
echo " Running PowerPC P1022 (e500v2) QEMU Test Suites  "
echo " CPU: e500v2 (PowerPC 32-bit Big Endian / SPE)    "
echo "=================================================="

BIN_DIR="${WORKSPACE_ROOT}/target/binaries"
REPORT_DIR="${WORKSPACE_ROOT}/target/reports"
mkdir -p "${REPORT_DIR}"

TESTS=(
    "c_baseline"
    "test_01_hello"
    "test_02_heap_alloc"
    "test_03_spe_float"
    "test_04_threads_sync"
    "test_05_timer_latency"
    "test_06_networking"
    "test_07_ffi"
    "test_08_https_rest"
    "mock_controller_poc"
    "cpp_benchmark"
    "rust_benchmark"
)

PASS_COUNT=0
FAIL_COUNT=0

for test in "${TESTS[@]}"; do
    BIN_PATH="${BIN_DIR}/${test}"
    LOG_PATH="${REPORT_DIR}/${test}.log"

    echo ""
    echo "=================================================="
    echo " [EXEC] Running: ${test} on QEMU (e500v2)"
    echo "=================================================="

    if [ ! -f "${BIN_PATH}" ]; then
        echo "  [FAIL] Binary ${BIN_PATH} does not exist!"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        continue
    fi

    # Execute under QEMU user-mode with e500v2 CPU model and target sysroot
    qemu-ppc-static -cpu e500v2 -L /usr/powerpc-linux-gnu "${BIN_PATH}" 2>&1 | tee "${LOG_PATH}"
    EXIT_CODE=${PIPESTATUS[0]}

    if [ ${EXIT_CODE} -eq 0 ]; then
        echo "  --> [PASS] ${test} (Exit: 0)"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        echo "  --> [FAIL] ${test} (Exit: ${EXIT_CODE})"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

echo ""
echo "=================================================="
echo " Test Suite Summary"
echo " Total: $((PASS_COUNT + FAIL_COUNT)) | Passed: ${PASS_COUNT} | Failed: ${FAIL_COUNT}"
echo "=================================================="

if [ ${FAIL_COUNT} -eq 0 ]; then
    echo " Result: ALL TESTS PASSED (GREEN)"
    exit 0
else
    echo " Result: SOME TESTS FAILED (RED)"
    exit 1
fi
