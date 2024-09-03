#!/bin/bash

RISCV_PATH="/opt/riscv"
OPENSBI_PATH="${RISCV_PATH}/opensbi/build/platform/generic/firmware/fw_jump.elf"
FREERTOS_PATH="${RISCV_PATH}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/build/RTOSDemo.axf"
VIOLET_PATH="target/qemu_virt/debug/qemu_freertos"

gdb-multiarch ${VIOLET_PATH} -ex "target remote localhost:12345"
#gdb-multiarch ${FREERTOS_PATH} -ex "target remote localhost:12345"
