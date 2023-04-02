FROM ubuntu:18.04

ENV RISCV=/opt/riscv
ENV PATH=$RISCV/bin:/root/.cargo/bin:$PATH
ENV MAKEFLAGS=-j4
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
WORKDIR $RISCV

# 基本ツールのインストール
RUN apt update && \
	apt install -y autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev pkg-config git libusb-1.0-0-dev device-tree-compiler default-jdk gnupg vim python3 cpio

# riscv-gnu-toolchainのビルド
RUN git clone https://github.com/riscv/riscv-gnu-toolchain.git && \
	cd riscv-gnu-toolchain && git checkout 2022.05.15 && \
	git submodule update --init --recursive
RUN cd riscv-gnu-toolchain && mkdir build && cd build && ../configure --prefix=${RISCV} --enable-multilib && make
RUN cd riscv-gnu-toolchain && mkdir build2 && cd build2 && ../configure --prefix=${RISCV} --enable-multilib && make linux

# QEMUのビルド
RUN apt update && \
	apt install -y pkg-config libglib2.0-dev libmount-dev python3 python3-pip python3-dev git libssl-dev libffi-dev build-essential automake libfreetype6-dev libtheora-dev libtool libvorbis-dev pkg-config texinfo zlib1g-dev unzip cmake yasm libx264-dev libmp3lame-dev libopus-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev pkg-config texinfo wget zlib1g-dev ninja-build libpixman-1-dev
RUN wget https://download.qemu.org/qemu-7.0.0.tar.xz && \
	tar xvJf qemu-7.0.0.tar.xz && \
	cd qemu-7.0.0 && \
	./configure --target-list=riscv32-softmmu,riscv64-softmmu --prefix=${RISCV} --enable-trace-backend=log && \
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

# Linuxの取得
RUN git clone https://github.com/torvalds/linux && \
	cd linux && \
	git checkout v5.17

# busybox(64bit)のビルド(ハイパーバイザ動作用)
RUN export ARCH=riscv && \
	export CROSS_COMPILE=riscv64-unknown-linux-gnu-  && \
	wget https://busybox.net/downloads/busybox-1.33.1.tar.bz2  && \
	tar -C . -xvf ./busybox-1.33.1.tar.bz2  && \
	mv ./busybox-1.33.1 ./busybox
	
# opensbiのビルド
RUN git clone https://github.com/riscv-software-src/opensbi.git && \
	cd opensbi && \
	git checkout 51113fe
