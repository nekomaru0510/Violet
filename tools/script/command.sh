#!/bin/bash
THISFILE_PATH=`dirname $0`
# 共通設定ファイルの読込み
source ${THISFILE_PATH}/header.sh

# 各種ツールのパス設定
OPENSBI_OUTPUT_FILE=`cd ${THISFILE_PATH} && bash -c 'source ./opensbi.sh && echo ${OUTPUT_FILE}'`
BUSYBOX_OUTPUT_FILE=`cd ${THISFILE_PATH} && bash -c 'source ./busybox.sh && echo ${OUTPUT_FILE}'`
LINUX_OUTPUT_FILE=`cd ${THISFILE_PATH} && bash -c 'source ./linux.sh && echo ${OUTPUT_FILE}'`

VIOLET_RLS_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/release"
VIOLET_DBG_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/debug"
VIOLET_BIN_PATH="${VIOLET_DBG_BIN_PATH}"
VIOLET_OUTPUT_FILE="${VIOLET_BIN_PATH}/sample"

NUM_OF_CPUS="2"
MEMORY_SIZE="2G"
QEMU_OPTIONS=""

function generate_qemu_option () {
    QEMU_OPTIONS="
        -cpu rv64 -M virt \
        -nographic \
        -m ${MEMORY_SIZE} \
        -smp ${NUM_OF_CPUS} \
        -bios ${OPENSBI_OUTPUT_FILE} \
        -kernel ${VIOLET_OUTPUT_FILE} \
        -initrd ${BUSYBOX_OUTPUT_FILE} \
        -append \"root=/dev/ram rdinit=/sbin/init console=ttyS0 mem=0x10000000\" \
        -device loader,file=${LINUX_OUTPUT_FILE},addr=0x90200000,force-raw=true \
        ${DEBUG_OPTION} "    
}

# Violet+Linuxの起動(Linuxの配置を変更)
function run_linux_with_violet () {
    generate_qemu_option
    eval qemu-system-riscv64 ${QEMU_OPTIONS}
}

function monitor_violet () {
    riscv64-unknown-elf-gdb ${VIOLET_OUTPUT_FILE} -x ${GDB_SCRIPTS_PATH}/connect
}

function install () {
    SCRIPTS_PATH=${SCRIPTS_PATH} TARGET=${1} bash -c 'source ${SCRIPTS_PATH}/${TARGET}.sh && install '
}

function build () {
    SCRIPTS_PATH=${SCRIPTS_PATH} TARGET=${1} bash -c 'source ${SCRIPTS_PATH}/${TARGET}.sh && build '
}

function monitor () {
    SCRIPTS_PATH=${SCRIPTS_PATH} TARGET=${1} bash -c 'source ${SCRIPTS_PATH}/${TARGET}.sh && monitor '
}

function help () {
    echo "Usage: command.sh [OPTION] targets"
    echo "OPTION"
    echo "-i    install targets"
    echo "-b    build targets"
    echo "-r    run"
    echo "-d    debug"
    echo "-t    test"
    echo "-m    monitor targets"
    echo "targets ... specify operation target"
    echo "ex) ./command.sh -b linux,opensbi "
    echo "    This command means 'build linux and opensbi' "
}

while getopts i:b:rdm:t:h OPT
do
    case $OPT in
        i)  
            list=(${OPTARG//,/ })
            for p in "${list[@]}"
            do
                install $p
            done
            ;;
        b)  
            list=(${OPTARG//,/ })
            for p in "${list[@]}"
            do
                build $p
            done
            ;;
        r)  
            run_linux_with_violet
            ;;
        d)  
            DEBUG_OPTION=${QEMU_DEBUG_OPTION}
            run_linux_with_violet
            ;;
        t)  
            VIOLET_OUTPUT_FILE="${OPTARG}"
            run_linux_with_violet
            ;;
        m)  
            list=(${OPTARG//,/ })
            for p in "${list[@]}"
            do
                monitor $p
            done
            ;;
        h)  help
            ;;
        \?) help
            ;;
    esac
done

