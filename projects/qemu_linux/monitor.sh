#!/bin/bash

RISCV_PATH="/opt/riscv"
OPENSBI_PATH="${RISCV_PATH}/opensbi/build/platform/generic/firmware/fw_jump.elf"
LINUX_PATH="${RISCV_PATH}/linux/vmlinux"

gdb-multiarch target/qemu_virt/debug/qemu_linux -x ../../tools/gdb/connect
#gdb-multiarch ${LINUX_PATH} -x ../../tools/gdb/connect
