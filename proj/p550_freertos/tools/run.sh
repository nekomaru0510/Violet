#!/bin/bash

PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd -- "${PROJECT_DIR}/../.." && pwd)"
cd "$PROJECT_DIR" || exit 1
pkill -f "openocd -f openocd_mcpu.cfg"
pkill -f "hifive_premier_p550/tools/start.sh"

RISCV_PATH="/opt/riscv"
FREERTOS_PATH="${RISCV_PATH}/FreeRTOS/FreeRTOS/Demo/RISC-V_RV64_SiFive_HiFive-Premier-P550_GCC/build/RTOSDemo.bin"

KERNEL_PATH="target/hifive_premier_p550/debug/p550_freertos"

riscv64-unknown-elf-objcopy -O binary ${KERNEL_PATH} ${KERNEL_PATH}.bin

"${REPO_ROOT}/env/hifive_premier_p550/tools/reset.sh" debug || exit 1
"${REPO_ROOT}/env/hifive_premier_p550/tools/start_openocd.sh" > /dev/null 2>&1 &
PID=$!

# Upload to hifive premier p550
gdb-multiarch ${KERNEL_PATH} -ex "target extended-remote localhost:3333" \
    -ex "set pagination off" \
    -ex "restore ${KERNEL_PATH}.bin binary 0x380200000" \
    -ex "restore ${FREERTOS_PATH} binary 0xC0000000" \
    -ex "continue" > /dev/null 2>&1 &

GDB_PID=$!

echo "Starting debug session..."
lsof -t /dev/ttyUSB2 | xargs --no-run-if-empty kill -9
minicom -D /dev/ttyUSB2 -b 115200

kill -9 $GDB_PID
kill -9 $PID
