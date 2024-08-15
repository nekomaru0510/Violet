#!/bin/bash

RISCV_PATH="/opt/riscv"
OPENSBI_PATH="${RISCV_PATH}/opensbi/build/platform/generic/firmware/fw_jump.elf"
FREERTOS_PATH="${RISCV_PATH}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/build/RTOSDemo.bin"

#riscv64-unknown-elf-gdb target/target/debug/qemu_freertos -x ../../tools/gdb/connect
gdb-multiarch target/riscv64imac-unknown-none-elf/debug/qemu_freertos -x ../../tools/gdb/connect