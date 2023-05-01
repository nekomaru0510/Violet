#!/bin/bash
THISFILE_PATH="$( cd "$( dirname "$BASH_SOURCE" )" && pwd -P )"
# 共通設定ファイルの読込み
source ${THISFILE_PATH}/header.sh

# 各種ツールのパス設定
TARGET_PATH="${RISCV_PATH}/opensbi"
OUTPUT_FILE="${TARGET_PATH}/build/platform/generic/firmware/fw_jump.elf"

# インストール
function install () {
    cd ${RISCV_PATH}

    git clone https://github.com/riscv-software-src/opensbi.git
	cd ${TARGET_PATH}
	git checkout 51113fe
}

# ビルド
function build () {
    cd ${TARGET_PATH}
    
    make clean
    # リンカスクリプトをクリアしないと、Violetのアドレスが変わらない
    rm ${TARGET_PATH}/build/platform/generic/firmware/fw_payload.elf.ld
    rm ${TARGET_PATH}/build/platform/generic/firmware/fw_jump.elf.ld
    # OpenSBIの開始アドレスは、0x8000_0000、Linuxの開始アドレスは、0x8020_0000。
    # [todo fix] 現状、Violetの開始アドレスは、0x8010_0000を設定。Violetのサイズが増えたら変更する必要性がでてくる
    make CROSS_COMPILE=riscv64-unknown-elf- PLATFORM=generic \
        FW_JUMP_ADDR=0x80100000
}

# モニタ
function monitor () {
    riscv64-unknown-elf-gdb ${TARGET_PATH}/build/platform/generic/firmware/fw_jump.elf -x ${GDB_SCRIPTS_PATH}/connect
}
