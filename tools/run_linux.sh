#!/bin/bash

# 各種ツールのパス設定
RISCV_PATH="/opt/riscv"
LINUX_PATH="/opt/riscv/linux"
#LINUX_PATH="/opt/riscv/aia/linux"
BUSYBOX_PATH="/opt/riscv/busybox"
OPENSBI_PATH="/opt/riscv/opensbi"
OPENSBI_BIN_PATH="${OPENSBI_PATH}/build/platform/generic/firmware"
VIOLET_PATH="/workspaces/Violet"
VIOLET_RLS_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/release"
VIOLET_DBG_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/debug"
#VIOLET_BIN_PATH="${VIOLET_RLS_BIN_PATH}"
VIOLET_BIN_PATH="${VIOLET_DBG_BIN_PATH}"

# Linuxの起動
function run_linux_only () {
    #qemu-system-riscv64 -cpu rv64 -M virt,aclint=on\
    qemu-system-riscv64 -cpu rv64 -M virt \
        -nographic \
        -kernel ${LINUX_PATH}/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_PATH}/rootfs.img \
        -append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        ${QEMU_DEBUG_OPTION}
}

# Violet+Linuxの起動
function run_linux_with_violet () {
    #qemu-system-riscv64 -cpu rv64 -M virt,aclint=on,aia=aplic \
    qemu-system-riscv64 -cpu rv64 -M virt \
        -nographic \
        -bios ${OPENSBI_BIN_PATH}/fw_jump.elf \
        -kernel ${LINUX_PATH}/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_PATH}/rootfs.img \
        -append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        -device loader,file=${VIOLET_BIN_PATH}/violet.bin,addr=0x80100000,force-raw=true \
        ${QEMU_DEBUG_OPTION}
        #-d mmu -D log.txt
        #-append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        #-device guest-loader,addr=0x80100000,kernel=${VIOLET_BIN_PATH}/Violet \
                #-cpu rv64,x-h=true \
}

function run_linux_with_violet_nosbi () {
    qemu-system-riscv64 -cpu rv64 -M virt -nographic  \
        -m 512M \
        -bios ${VIOLET_BIN_PATH}/violet \
        -kernel ${LINUX_PATH}/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_PATH}/rootfs.img \
        -append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        ${QEMU_DEBUG_OPTION}
}

# デバッグオプション
QEMU_DEBUG_OPTION=""

if [ $# -eq 0 ]; then
    run_linux_with_violet
elif [ $1 == "-d" ]; then
    QEMU_DEBUG_OPTION="-gdb tcp::12345 -S"
    run_linux_with_violet
elif [ $1 == "-lo" ]; then
    run_linux_only
fi
