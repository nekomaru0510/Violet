#!/bin/bash
THISFILE_PATH="$( cd "$( dirname "$BASH_SOURCE" )" && pwd -P )"
# 共通設定ファイルの読込み
source ${THISFILE_PATH}/header.sh

# 各種ツールのパス設定
TARGET_PATH="${RISCV_PATH}/FreeRTOS"
OUTPUT_FILE="${TARGET_PATH}/"

BUSYBOX_OUTPUT_FILE=`bash -c "source ${THISFILE_PATH}/busybox.sh && echo '${OUTPUT_FILE}'"`

# インストール
function install () {
    cd ${RISCV_PATH}
    git clone https://github.com/FreeRTOS/FreeRTOS.git
	cd ${TARGET_PATH}
	git checkout 51113fe
    git submodule update --init --recursive
    cp -r ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt_GCC ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC 
    cd ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC
    sed -i -e "s/rv32imac/rv64imac/g" Makefile
    sed -i -e "s/ilp32/lp64/g" Makefile
}

# ビルド
function build () {
    cd ${TARGET_PATH}/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC 
    make 
    riscv64-unknown-elf-objcopy -O binary build/RTOSDemo.axf build/RTOSDemo.bin
}
