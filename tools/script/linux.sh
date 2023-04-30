#!/bin/bash

# 共通設定ファイルの読込み
source ./header.sh

# 各種ツールのパス設定
TARGET_PATH="${RISCV_PATH}/linux"
OUTPUT_FILE="${TARGET_PATH}/vmlinux.bin"

BUSYBOX_OUTPUT_FILE=`bash -c 'source ./busybox.sh && echo ${OUTPUT_FILE}'`

# インストール
function install () {
    cd ${RISCV_PATH}
    git clone https://github.com/torvalds/linux
	cd ${TARGET_PATH}
	git checkout v5.17
}

# ビルド
function build () {
    cd ${TARGET_PATH}
    
    # Linuxのビルド
    make ARCH=riscv CROSS_COMPILE=riscv64-unknown-linux-gnu- defconfig
	make ARCH=riscv CROSS_COMPILE=riscv64-unknown-linux-gnu- -j 2
    riscv64-unknown-elf-objcopy -O binary \
        ${TARGET_PATH}/vmlinux \
        ${TARGET_PATH}/vmlinux.bin
    
    # シンボリックリンクの生成
    ln -s ${TARGET_PATH}/vmlinux.bin ${OUTPUT_FILE}
}

# 単体実行
function run () {
    #qemu-system-riscv64 -cpu rv64 -M virt,aclint=on\
    qemu-system-riscv64 -cpu rv64 -M virt \
        -nographic \
        -kernel ${TARGET_PATH}/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_OUTPUT_FILE} \
        -append "root=/dev/ram rdinit=/sbin/init console=ttyS0 mem=0x10000000" \
        -smp 2
        #-append "root=/dev/ram console=ttyS0"
}

# デバッグ
function debug () {
    #qemu-system-riscv64 -cpu rv64 -M virt,aclint=on\
    qemu-system-riscv64 -cpu rv64 -M virt \
        -nographic \
        -kernel ${TARGET_PATH}/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_OUTPUT_FILE} \
        -append "root=/dev/ram rdinit=/sbin/init console=ttyS0 mem=0x10000000" \
        -smp 2 \
        -gdb tcp::12345 -S
        #-append "root=/dev/ram console=ttyS0" \
}

# モニタ
function monitor () {
    riscv64-unknown-elf-gdb ${TARGET_PATH}/vmlinux -x ${GDB_SCRIPTS_PATH}/connect
}
