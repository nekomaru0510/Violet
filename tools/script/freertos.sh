#!/bin/bash
THISFILE_PATH="$( cd "$( dirname "$BASH_SOURCE" )" && pwd -P )"
# 共通設定ファイルの読込み
source ${THISFILE_PATH}/header.sh

# 各種ツールのパス設定
TARGET_PATH="${RISCV_PATH}/FreeRTOS"
OUTPUT_FILE="${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/build/RTOSDemo.bin"

BUSYBOX_OUTPUT_FILE=`bash -c "source ${THISFILE_PATH}/busybox.sh && echo '${OUTPUT_FILE}'"`

# インストール
function install () {
    cd ${RISCV_PATH}
    git clone https://github.com/FreeRTOS/FreeRTOS.git
	cd ${TARGET_PATH}
	git checkout 82099c32a0d5960685c79033edde8f381c2f73ea
    git submodule update --init --recursive FreeRTOS/Source
    cp -r ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt_GCC ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC 
    cd ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC
    sed -i -e "s/32/64/g" main_blinky.c
    sed -i -e "s/rv32imac/rv64imac/g" Makefile
    sed -i -e "s/ilp32/lp64/g" Makefile
    sed -i -e "s/ORIGIN = 0x80000000/ORIGIN = 0xc0000000/g" fake_rom.lds
    sed -i -e "s/ORIGIN = 0x80080000/ORIGIN = 0xc0080000/g" fake_rom.lds
}

# ビルド
function build () {
    cd ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC 
    make clean
    make DEBUG=1
    riscv64-unknown-elf-objcopy -O binary build/RTOSDemo.axf build/RTOSDemo.bin
}

# モニタ
function monitor () {
    riscv64-unknown-elf-gdb ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/build/RTOSDemo.axf -x ${GDB_SCRIPTS_PATH}/connect
}