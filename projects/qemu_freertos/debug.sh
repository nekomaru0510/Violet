#!/bin/bash

RISCV_PATH="/opt/riscv"
OPENSBI_PATH="${RISCV_PATH}/opensbi/build/platform/generic/firmware/fw_jump.elf"
FREERTOS_PATH="${RISCV_PATH}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/build/RTOSDemo.bin"

qemu-system-riscv64 \
    -cpu rv64 -M virt \
    -nographic \
    -m 2G \
    -smp 2 \
    -bios ${OPENSBI_PATH} \
    -append 'root=/dev/ram rdinit=/sbin/init console=ttyS0 mem=0x10000000' \
    -device loader,file=${FREERTOS_PATH},addr=0xC0000000,force-raw=true \
    -gdb tcp::12345 -S \
    -kernel target/riscv64imac-unknown-none-elf/debug/qemu_freertos \