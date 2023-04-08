# 共通設定ファイル
VIOLET_PATH=$(dirname $(dirname $(cd $(dirname $0);pwd)))
RISCV_PATH="/opt/riscv"
SCRIPTS_PATH="${VIOLET_PATH}/tools/script"
GDB_SCRIPTS_PATH="${VIOLET_PATH}/tools/gdb"
QEMU_DEBUG_OPTION="-gdb tcp::12345 -S"

