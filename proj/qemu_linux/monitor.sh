#!/bin/bash

RISCV_PATH="/opt/riscv"
OPENSBI_PATH="${RISCV_PATH}/opensbi/build/platform/generic/firmware/fw_jump.elf"
LINUX_PATH="${RISCV_PATH}/linux/vmlinux"
VIOLET_PATH="target/qemu_virt/debug/qemu_linux"

gdb-multiarch ${VIOLET_PATH} -ex "target remote localhost:12345"