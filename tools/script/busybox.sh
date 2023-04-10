#!/bin/bash

# 共通設定ファイルの読込み
source ./header.sh

# 各種ツールのパス設定
TARGET_PATH="${RISCV_PATH}/busybox"
OUTPUT_FILE="${TARGET_PATH}/rootfs.img"

# インストール
function install () {
    cd ${RISCV_PATH}

    export ARCH=riscv
	export CROSS_COMPILE=riscv64-unknown-linux-gnu-
	git clone https://github.com/mirror/busybox.git
	cd ${TARGET_PATH}
	git checkout 1_33_2
	make defconfig
	sed -i -e "s/# CONFIG_STATIC is not set/CONFIG_STATIC=y/g" .config
}

# ビルド
function build () {
    cd ${RISCV_PATH}

    export ARCH=riscv
	export CROSS_COMPILE=riscv64-unknown-linux-gnu-
	mkdir -p ${TARGET_PATH}/_install/etc/init.d
    cp ${VIOLET_PATH}/config/busybox/fstab ${TARGET_PATH}/_install/etc/fstab
    cp ${VIOLET_PATH}/config/busybox/rcS ${TARGET_PATH}/_install/etc/init.d/rcS
    make -C busybox install
	mkdir -p busybox/_install/etc/init.d
	mkdir -p busybox/_install/dev
	mkdir -p busybox/_install/proc
	mkdir -p busybox/_install/sys
	mkdir -p busybox/_install/apps
	cd busybox/_install; find ./ | cpio -o -H newc > ../rootfs.img
}
