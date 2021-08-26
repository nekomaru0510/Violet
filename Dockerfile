FROM ubuntu:18.04

ENV RISCV=/opt/riscv
ENV PATH=$RISCV/bin:/root/.cargo/bin:$PATH
ENV MAKEFLAGS=-j4
WORKDIR $RISCV

# 基本ツールのインストール
RUN apt update && \
	apt install -y autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev pkg-config git libusb-1.0-0-dev device-tree-compiler default-jdk gnupg vim python3 fish

# riscv-gnu-toolchain(ベクトル対応ver.)のビルド
RUN git clone -b rvv-0.9.x --single-branch https://github.com/riscv/riscv-gnu-toolchain.git && \
	cd riscv-gnu-toolchain && git checkout 5842fde8ee5bb3371643b60ed34906eff7a5fa31 && \
	git submodule update --init --recursive
RUN cd riscv-gnu-toolchain && mkdir build && cd build && ../configure --prefix=${RISCV} --enable-multilib && make

# QEMUのビルド
RUN apt update && \
	apt install -y pkg-config libglib2.0-dev libmount-dev python3 python3-pip python3-dev git libssl-dev libffi-dev build-essential automake libfreetype6-dev libtheora-dev libtool libvorbis-dev pkg-config texinfo zlib1g-dev unzip cmake yasm libx264-dev libmp3lame-dev libopus-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev pkg-config texinfo wget zlib1g-dev ninja-build libpixman-1-dev
RUN wget https://download.qemu.org/qemu-5.0.0.tar.xz && \
	tar xvJf qemu-5.0.0.tar.xz && \
	cd qemu-5.0.0 && \
	./configure --target-list=riscv32-softmmu,riscv64-softmmu --prefix=${RISCV} && \
	make -j 2 && \
	make install

# Rustのインストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 

RUN chsh -s /usr/bin/fish
