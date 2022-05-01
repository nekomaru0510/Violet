#!/bin/bash

# 各種ツールのパス設定
RISCV_PATH="/opt/riscv"
LINUX_PATH="/opt/riscv/linux"
BUSYBOX_PATH="/opt/riscv/busybox"
OPENSBI_PATH="/opt/riscv/opensbi"
OPENSBI_BIN_PATH="${OPENSBI_PATH}/build/platform/generic/firmware"
VIOLET_PATH="/workspaces/Violet"
VIOLET_RLS_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/release"
VIOLET_DBG_BIN_PATH="${VIOLET_PATH}/target/riscv64imac-unknown-none-elf/debug"
#VIOLET_BIN_PATH="${VIOLET_RLS_BIN_PATH}"
VIOLET_BIN_PATH="${VIOLET_DBG_BIN_PATH}"

# Busyboxのビルド(ゲストLinux用のrootfs作成)
function build_busybox () {
    cd ${RISCV_PATH}
    export ARCH=riscv
	export CROSS_COMPILE=riscv64-unknown-linux-gnu-
    cp -f ${VIOLET_PATH}/config/busybox/busybox-1.33.1_defconfig busybox/.config  && \
	make -C busybox oldconfig
	make -C busybox install
	mkdir -p busybox/_install/etc/init.d
	mkdir -p busybox/_install/dev
	mkdir -p busybox/_install/proc
	mkdir -p busybox/_install/sys
	mkdir -p busybox/_install/apps
	ln -sf /sbin/init busybox/_install/init
	cd busybox/_install; find ./ | cpio -o -H newc > ../rootfs.img
}

# OpenSBIのビルド(Linuxのみ動作させる用)
function build_opensbi () {
    cd ${OPENSBI_PATH}
    make clean
    # リンカスクリプトをクリアしないと、Violetのアドレスが変わらない
    rm ${OPENSBI_BIN_PATH}/fw_payload.elf.ld
    rm ${OPENSBI_BIN_PATH}/fw_jump.elf.ld
    # ビルド
    make CROSS_COMPILE=riscv64-unknown-elf- PLATFORM=generic
}

# OpenSBIのビルド(Violet挟む用)
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


# Linuxのビルド
function build_linux () {
    cd ${LINUX_PATH}
	make ARCH=riscv CROSS_COMPILE=riscv64-unknown-linux-gnu- defconfig
	make ARCH=riscv CROSS_COMPILE=riscv64-unknown-linux-gnu- -j 2
}

if [ $# -eq 0 ]; then
    echo "please specify build project"
elif [ $1 == "-v" ]; then
    build_violet
elif [ $1 == "-o" ]; then
    build_opensbi_for_Violet
elif [ $1 == "-l" ]; then
    build_opensbi_for_Violet
elif [ $1 == "-b" ]; then    
    build_busybox
fi
