#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

IMAGE_NAME="legacy_ppc_rust_env:latest"

echo "=================================================="
echo " Step 1: Building Docker Image (${IMAGE_NAME})"
echo "=================================================="

docker build \
    --platform linux/amd64 \
    -t "${IMAGE_NAME}" \
    -f "${WORKSPACE_ROOT}/docker/Dockerfile" \
    "${WORKSPACE_ROOT}"

echo "=================================================="
echo " Step 2: Running Build & QEMU Tests inside Docker"
echo "=================================================="

docker run --rm \
    --platform linux/amd64 \
    -v "${WORKSPACE_ROOT}:/workspace" \
    -w /workspace \
    "${IMAGE_NAME}" \
    bash -c "
        chmod +x scripts/*.sh
        ./scripts/build_all.sh
        ./scripts/run_all_tests.sh
    "

echo "=================================================="
echo " Verification Pipeline Completed Successfully"
echo "=================================================="
