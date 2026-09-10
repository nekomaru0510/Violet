#!/bin/bash

PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd -- "${PROJECT_DIR}/../.." && pwd)"
cd "$PROJECT_DIR" || exit 1

RISCV_PATH="/opt/riscv"
DEPLOY_IMAGE_PATH="${RISCV_PATH}/yocto/poky/dist/build/tmp-glibc/deploy/images/hifive-premier-p550"

KERNEL_PATH="target/hifive_premier_p550/debug/p550_bench"

BENCH_PATH="${REPO_ROOT}/benchmarks/interrupt-bench/target/hifive_premier_p550/debug/interrupt-bench"

riscv64-unknown-elf-objcopy -O binary ${KERNEL_PATH} ${KERNEL_PATH}.bin
riscv64-unknown-elf-objcopy -O binary ${BENCH_PATH} ${BENCH_PATH}.bin

"${REPO_ROOT}/env/hifive_premier_p550/tools/reset.sh" debug || exit 1
"${REPO_ROOT}/env/hifive_premier_p550/tools/start_openocd.sh" > /dev/null 2>&1 &
PID=$!

# Upload to hifive premier p550
gdb-multiarch ${KERNEL_PATH} -ex "target extended-remote localhost:3333" \
    -ex "set pagination off" \
    -ex "restore ${KERNEL_PATH}.bin binary 0x380200000" \
    -ex "restore ${BENCH_PATH}.bin binary 0xc0000000" \
    -ex "continue" \
    -ex "detach" \
    -ex "q" > /dev/null 2>&1 &

minicom -D /dev/ttyUSB2 -b 115200

kill -9 $PID
