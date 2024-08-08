FROM ubuntu:22.04

ENV RISCV=/opt/riscv
ENV PATH=$RISCV/bin:/root/.cargo/bin:$PATH
ENV MAKEFLAGS=-j4
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
WORKDIR $RISCV

# ============ Basic Environment Violet ============

# Install Tools
RUN apt update && \
	apt install -y autoconf automake autotools-dev curl bc git device-tree-compiler vim python3 cpio

# Build QEMU
RUN apt update && \
	apt install -y pkg-config libglib2.0-dev libmount-dev python3 python3-pip python3-dev git libssl-dev libffi-dev build-essential automake libfreetype6-dev libtheora-dev libtool libvorbis-dev pkg-config texinfo zlib1g-dev unzip cmake yasm libx264-dev libmp3lame-dev libopus-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev pkg-config texinfo wget zlib1g-dev ninja-build libpixman-1-dev
RUN wget https://download.qemu.org/qemu-8.0.0.tar.xz && \
	tar xvJf qemu-8.0.0.tar.xz && \
	rm qemu-8.0.0.tar.xz && \
	cd qemu-8.0.0 && \
	./configure --target-list=riscv32-softmmu,riscv64-softmmu --prefix=${RISCV} && \
	make -j 2 && \
	make install

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Build OpenSBI
RUN cd ${RISCV} && \
	git clone https://github.com/riscv-software-src/opensbi.git  && \
	cd ${RISCV}/opensbi && \
	git checkout 51113fe
RUN	cd ${RISCV}/opensbi && \
	apt install -y gcc-riscv64-linux-gnu && \
	make CROSS_COMPILE=riscv64-linux-gnu- PLATFORM=generic FW_JUMP_ADDR=0x80100000

# ============ GuestOS ============

# Build Linux
RUN cd ${RISCV} && \
    apt install -y autoconf automake autotools-dev curl \
		libmpc-dev libmpfr-dev libgmp-dev gawk build-essential \
		bison flex texinfo gperf libtool patchutils bc zlib1g-dev \
		libexpat-dev pkg-config libusb-1.0-0-dev device-tree-compiler default-jdk gnupg && \
    git clone https://github.com/torvalds/linux -b v5.17 --depth 1
RUN	cd ${RISCV}/linux && \
	make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- defconfig  && \
	make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- -j 2 && \
	riscv64-linux-gnu-objcopy -O binary vmlinux vmlinux.bin

# Build Busybox
RUN cd ${RISCV} && \
	export ARCH=riscv && \
	export CROSS_COMPILE=riscv64-linux-gnu- && \
	git clone https://github.com/mirror/busybox.git && \
	cd ${RISCV}/busybox && \
	git checkout 1_33_2 && \
	make defconfig && \
	sed -i -e "s/# CONFIG_STATIC is not set/CONFIG_STATIC=y/g" .config	
RUN	cd ${RISCV}/busybox && \
    export ARCH=riscv && \
	export CROSS_COMPILE=riscv64-linux-gnu- && \
	mkdir -p _install/etc/init.d && \
	echo "\
proc    /proc   proc    defaults    0   0 \n\
sysfs   /sys    sysfs   defaults    0   0 " \
	> _install/etc/fstab && \
	echo "#!/bin/sh \n\
\n\
/bin/mount -a \n\
mkdir -p /dev \n\
/bin/mount -t devtmpfs devtmpfs /dev" \
	> _install/etc/init.d/rcS && \
	chmod +x _install/etc/init.d/rcS && \
	make install; \
	mkdir -p _install/etc/init.d && \
	mkdir -p _install/dev && \
	mkdir -p _install/proc && \
	mkdir -p _install/sys && \
	mkdir -p _install/apps && \
	cd _install &&\
	find ./ | cpio -o -H newc > ../rootfs.img
