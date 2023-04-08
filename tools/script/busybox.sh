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
	wget https://busybox.net/downloads/busybox-1.33.1.tar.bz2
	tar -C . -xvf ./busybox-1.33.1.tar.bz2
	mv ./busybox-1.33.1 ./busybox
}

# ビルド
function build () {
    cd ${RISCV_PATH}

    export ARCH=riscv
	export CROSS_COMPILE=riscv64-unknown-linux-gnu-
    cp -f ${VIOLET_PATH}/config/busybox/busybox-1.33.1_defconfig busybox/.config
    cp ${VIOLET_PATH}/config/busybox/fstab ${BUSYBOX_PATH}/_install/etc/fstab
    cp ${VIOLET_PATH}/config/busybox/rcS ${BUSYBOX_PATH}/_install/etc/init.d/rcS
    make -C busybox install
	mkdir -p busybox/_install/etc/init.d
	mkdir -p busybox/_install/dev
	mkdir -p busybox/_install/proc
	mkdir -p busybox/_install/sys
	mkdir -p busybox/_install/apps
	ln -sf /sbin/init busybox/_install/init
	cd busybox/_install; find ./ | cpio -o -H newc > ../rootfs.img
}
