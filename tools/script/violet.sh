#!/bin/bash

# 共通設定ファイルの読込み
source ./header.sh

# 各種ツールのパス設定
TARGET_PATH="${VIOLET_PATH}"
OUTPUT_FILE="${TARGET_PATH}/target/riscv64imac-unknown-none-elf/debug/sample"

# インストール
function install () {
    echo "already installed"
}

# ビルド
function build () {
    cd ${TARGET_PATH}
    cargo make build
}

# モニタ
function monitor () {
    riscv64-unknown-elf-gdb ${OUTPUT_FILE} -x ${GDB_SCRIPTS_PATH}/connect
}
