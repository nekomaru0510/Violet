FROM ubuntu:24.04

ENV RISCV=/opt/riscv
ENV PATH=$RISCV/bin:/root/.cargo/bin:$PATH
ENV MAKEFLAGS=-j4
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
WORKDIR $RISCV

# 基本ツールのインストール
RUN apt update && \
	apt install -y autoconf automake autotools-dev curl bc git device-tree-compiler vim python3 cpio

# QEMUのビルド
RUN apt update && \
	apt install -y pkg-config libglib2.0-dev libmount-dev python3 python3-pip python3-dev git libssl-dev libffi-dev build-essential automake libfreetype6-dev libtheora-dev libtool libvorbis-dev pkg-config texinfo zlib1g-dev unzip cmake yasm libx264-dev libmp3lame-dev libopus-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev pkg-config texinfo wget zlib1g-dev ninja-build libpixman-1-dev
RUN wget https://download.qemu.org/qemu-8.0.0.tar.xz && \
	tar xvJf qemu-8.0.0.tar.xz && \
	cd qemu-8.0.0 && \
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
