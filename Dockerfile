FROM ubuntu:18.04

ENV RISCV=/opt/riscv
ENV PATH=$RISCV/bin:/root/.cargo/bin:$PATH
ENV MAKEFLAGS=-j4
WORKDIR $RISCV

# 基本ツールのインストール
RUN apt update && \
	apt install -y autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev pkg-config git libusb-1.0-0-dev device-tree-compiler default-jdk gnupg vim python3 fish

# riscv-gnu-toolchainのビルド
RUN git clone -b rvv-0.9.x --single-branch https://github.com/riscv/riscv-gnu-toolchain.git && \
	cd riscv-gnu-toolchain && git checkout 5842fde8ee5bb3371643b60ed34906eff7a5fa31 && \
	git submodule update --init --recursive
RUN cd riscv-gnu-toolchain && mkdir build && cd build && ../configure --prefix=${RISCV} --enable-multilib && make
RUN cd riscv-gnu-toolchain && mkdir build2 && cd build2 && ../configure --prefix=${RISCV} --enable-multilib && make linux

# QEMUのビルド
RUN apt update && \
	apt install -y pkg-config libglib2.0-dev libmount-dev python3 python3-pip python3-dev git libssl-dev libffi-dev build-essential automake libfreetype6-dev libtheora-dev libtool libvorbis-dev pkg-config texinfo zlib1g-dev unzip cmake yasm libx264-dev libmp3lame-dev libopus-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev pkg-config texinfo wget zlib1g-dev ninja-build libpixman-1-dev
RUN wget https://download.qemu.org/qemu-7.0.0.tar.xz && \
	tar xvJf qemu-7.0.0.tar.xz && \
	cd qemu-7.0.0 && \
	./configure --target-list=riscv32-softmmu,riscv64-softmmu --prefix=${RISCV} && \
	make -j 2 && \
	make install

# Rustのインストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# cargo-makeのインストール [todo fix] cargo installからインストールできるようにする
RUN wget https://github.com/sagiegurari/cargo-make/releases/download/0.35.11/cargo-make-v0.35.11-x86_64-unknown-linux-musl.zip && \
	unzip cargo-make-v0.35.11-x86_64-unknown-linux-musl.zip && \
	cd cargo-make-v0.35.11-x86_64-unknown-linux-musl && \
	cp cargo-make /root/.cargo/bin/ && \
	cp makers /root/.cargo/bin/

# シェルの変更
RUN chsh -s /usr/bin/fish

# Linuxの取得
RUN git clone https://github.com/torvalds/linux && \
	cd linux && \
	git checkout v5.17
	
# Linuxのビルド	(ハイパーバイザ動作用)
RUN	cd linux && \
	make ARCH=riscv CROSS_COMPILE=riscv64-unknown-linux-gnu- defconfig && \
	make ARCH=riscv CROSS_COMPILE=riscv64-unknown-linux-gnu- -j 2
	# デバッグする場合は、CONFIG_DEBUG_INFOを有効にする

RUN apt update && \
	apt install -y cpio

# busybox(64bit)のビルド(ハイパーバイザ動作用)
RUN export ARCH=riscv && \
	export CROSS_COMPILE=riscv64-unknown-linux-gnu-  && \
	wget https://busybox.net/downloads/busybox-1.33.1.tar.bz2  && \
	tar -C . -xvf ./busybox-1.33.1.tar.bz2  && \
	mv ./busybox-1.33.1 ./busybox  && \	
	make -C busybox defconfig  && \
	make -C busybox install && \
	mkdir -p busybox/_install/etc/init.d && \
	mkdir -p busybox/_install/dev && \
	mkdir -p busybox/_install/proc && \
	mkdir -p busybox/_install/sys && \
	mkdir -p busybox/_install/apps && \
	ln -sf /sbin/init busybox/_install/init && \
	cd busybox/_install; find ./ | cpio -o -H newc > ../rootfs.img
	#git clone https://github.com/kvm-riscv/howto.git  && \
	#cp -f ./howto/configs/busybox-1.33.1_defconfig busybox/.config  && \
	
# opensbiのビルド
RUN git clone https://github.com/riscv-software-src/opensbi.git && \
	cd opensbi && \
	git checkout 51113fe && \
	make CROSS_COMPILE=riscv64-unknown-elf- PLATFORM=generic
