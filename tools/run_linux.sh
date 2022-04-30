#!/bin/bash




# 各種ツールのパス設定
LINUX_PATH="/opt/riscv"
BUSYBOX_PATH="/opt/riscv/busybox"
OPENSBI_PATH="/opt/riscv/opensbi"
OPENSBI_BIN_PATH="${OPENSBI_PATH}/build/platform/generic/firmware"
VIOLET_PATH="/workspaces/Violet"
VIOLET_RLS_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/release"
VIOLET_DBG_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/debug"
#VIOLET_BIN_PATH="${VIOLET_RLS_BIN_PATH}"
VIOLET_BIN_PATH="${VIOLET_DBG_BIN_PATH}"



# OpenSBIのビルド(Linuxのみ動作させる用)
function build_opensbi () {
    cd ${OPENSBI_PATH}
    make clean
    # リンカスクリプトをクリアしないと、Violetのアドレスが変わらない
    rm ${OPENSBI_BIN_PATH}/fw_payload.elf.ld
    rm ${OPENSBI_BIN_PATH}/fw_jump.elf.ld
    # OpenSBIの開始アドレスは、0x8000_0000、Linuxの開始アドレスは、0x8020_0000。
    # 現状、Violetの開始アドレスは、0x8010_0000を設定。Violetのサイズが増えたら変更する必要性がでてくる
    make CROSS_COMPILE=riscv64-unknown-elf- PLATFORM=generic
    #    FW_PAYLOAD_PATH=${VIOLET_BIN_PATH}/Violet.bin \
    #    FW_PAYLOAD_OFFSET=0x100000
}

# OpenSBIのビルド2(Violet挟む用)
function build_opensbi_for_Violet () {
    cd ${OPENSBI_PATH}
    make clean
    # リンカスクリプトをクリアしないと、Violetのアドレスが変わらない
    rm ${OPENSBI_BIN_PATH}/fw_payload.elf.ld
    rm ${OPENSBI_BIN_PATH}/fw_jump.elf.ld
    # OpenSBIの開始アドレスは、0x8000_0000、Linuxの開始アドレスは、0x8020_0000。
    # 現状、Violetの開始アドレスは、0x8010_0000を設定。Violetのサイズが増えたら変更する必要性がでてくる
    make CROSS_COMPILE=riscv64-unknown-elf- PLATFORM=generic \
        FW_JUMP_ADDR=0x80100000
}


# Violetのビルド
function build_violet () {
    cd ${VIOLET_PATH}
    # cargo build --release
    cargo build
    riscv64-unknown-elf-objcopy -O binary \
        ${VIOLET_BIN_PATH}/violet \
        ${VIOLET_BIN_PATH}/violet.bin
}

# Linuxの起動
function run_linux_only () {
    qemu-system-riscv64 -nographic -machine virt \
        -bios ${OPENSBI_BIN_PATH}/fw_jump.elf \
        -kernel ${LINUX_PATH}/linux/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_PATH}/rootfs.img \
        -append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        ${QEMU_DEBUG_OPTION}
}

# Violet+Linuxの起動2
function run_linux_with_violet () {
    qemu-system-riscv64 -cpu rv64 -M virt -nographic  \
        -bios ${OPENSBI_BIN_PATH}/fw_jump.elf \
        -kernel ${LINUX_PATH}/linux/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_PATH}/rootfs.img \
        -append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        -device loader,file=${VIOLET_BIN_PATH}/violet.bin,addr=0x80100000,force-raw=true \
        ${QEMU_DEBUG_OPTION}
        #-device guest-loader,addr=0x80100000,kernel=${VIOLET_BIN_PATH}/Violet \
                #-cpu rv64,x-h=true \
}

function run_linux_with_violet_nosbi () {
    qemu-system-riscv64 -cpu rv64 -M virt -nographic  \
        -m 512M \
        -bios ${VIOLET_BIN_PATH}/violet \
        -kernel ${LINUX_PATH}/linux/arch/riscv/boot/Image \
        -initrd ${BUSYBOX_PATH}/rootfs.img \
        -append "root=/dev/ram rdinit=/bin/sh console=ttyS0" \
        ${QEMU_DEBUG_OPTION}
}

# デバッグオプション
QEMU_DEBUG_OPTION=""

if [ $# -eq 0 ]; then
    run_linux_with_violet
elif [ $1 == "-b" ]; then
    build_violet
elif [ $1 == "-d" ]; then
    QEMU_DEBUG_OPTION="-gdb tcp::12345 -S"
    run_linux_with_violet
elif [ $1 == "-lo" ]; then
    run_linux_only
fi
