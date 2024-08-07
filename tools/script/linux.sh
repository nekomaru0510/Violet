#!/bin/bash
THISFILE_PATH="$( cd "$( dirname "$BASH_SOURCE" )" && pwd -P )"
# 共通設定ファイルの読込み
source ${THISFILE_PATH}/header.sh

# 各種ツールのパス設定
TARGET_PATH="${RISCV_PATH}/linux"
OUTPUT_FILE="${TARGET_PATH}/vmlinux.bin"

BUSYBOX_OUTPUT_FILE=`bash -c "source ${THISFILE_PATH}/busybox.sh && echo '${OUTPUT_FILE}'"`

# インストール
function install () {
    cd ${RISCV_PATH}
    apt install -y autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev pkg-config libusb-1.0-0-dev device-tree-compiler default-jdk gnupg
    apt install -y gcc-riscv64-linux-gnu
    git clone https://github.com/torvalds/linux -b v5.17 --depth 1
	cd ${TARGET_PATH}
	#git checkout v5.17
}

# ビルド
function build () {
    cd ${TARGET_PATH}
    
    # Linuxのビルド
    make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- defconfig
	make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- -j 2
    riscv64-linux-gnu-objcopy -O binary \
        ${TARGET_PATH}/vmlinux \
        ${TARGET_PATH}/vmlinux.bin
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
    riscv64-linux-gnu-gdb ${TARGET_PATH}/vmlinux -x ${GDB_SCRIPTS_PATH}/connect
}
