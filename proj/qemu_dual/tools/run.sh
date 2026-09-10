#!/bin/bash

RISCV_PATH="/opt/riscv"
OPENSBI_PATH="${OPENSBI_PATH:-${RISCV_PATH}/opensbi/build/platform/generic/firmware/fw_jump.elf}"
BUSYBOX_PATH="${RISCV_PATH}/busybox/rootfs.img"
FREERTOS_PATH="${RISCV_PATH}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/build/RTOSDemo.bin"
LINUX_PATH="${RISCV_PATH}/linux/vmlinux.bin"

qemu-system-riscv64 \
    -cpu rv64 -M virt \
    -nographic \
    -m 2G \
    -smp 2 \
    -bios ${OPENSBI_PATH} \
    -initrd ${BUSYBOX_PATH}  \
    -append 'root=/dev/ram rdinit=/sbin/init console=ttyS0 mem=0x10000000' \
    -device loader,file=${LINUX_PATH},addr=0x90200000,force-raw=true  \
    -device loader,file=${FREERTOS_PATH},addr=0xc0000000,force-raw=true \
    -kernel $1 \