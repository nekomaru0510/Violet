# All-in-one environment (so it takes a long time to build)
FROM ubuntu:24.04

ENV RISCV=/opt/riscv
ENV PATH=$RISCV/bin:/root/.cargo/bin:$PATH
ENV MAKEFLAGS=-j4
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

ENV DEBIAN_FRONTEND noninteractive

WORKDIR $RISCV

# ============ Basic Environment Violet ============

# Install Tools
RUN apt update && \
	apt install -y autoconf automake autotools-dev curl bc git device-tree-compiler vim python3 cpio gdb-multiarch

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
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	rustup install nightly-2024-07-25-x86_64-unknown-linux-gnu && \
	rustup component add rust-src --toolchain nightly-2024-07-25-x86_64-unknown-linux-gnu

# Build OpenSBI
RUN cd ${RISCV} && \
	git clone https://github.com/riscv-software-src/opensbi.git  && \
	cd ${RISCV}/opensbi && \
	git checkout 51113fe
RUN	cd ${RISCV}/opensbi && \
	apt install -y gcc-riscv64-linux-gnu && \
	make CROSS_COMPILE=riscv64-linux-gnu- PLATFORM=generic FW_JUMP_ADDR=0x80100000 PLATFORM_RISCV_ISA=rv64imafdc_zifencei

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
	sed -i -e "s/# CONFIG_STATIC is not set/CONFIG_STATIC=y/g" .config && \
	# Disable TC (Traffic Control) features to avoid CBQ compilation errors
	sed -i 's/CONFIG_TC=y/# CONFIG_TC is not set/' .config && \
	sed -i 's/CONFIG_FEATURE_TC_INGRESS=y/# CONFIG_FEATURE_TC_INGRESS is not set/' .config
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

# Build FreeRTOS
RUN cd ${RISCV} && \
	git clone https://github.com/petitstrawberry/FreeRTOS.git && \
	cd ${RISCV}/FreeRTOS && \
	git checkout 5903bc9e1b42d0d8cc9a56dae03128f2b86208e1 && \
	git submodule update --init --recursive FreeRTOS/Source && \
	cp -r ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt_GCC ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC  && \
	cd ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC && \
	apt install -y gcc-riscv64-unknown-elf picolibc-riscv64-unknown-elf && \
	sed -i -e "s/32/64/g" main_blinky.c && \
	sed -i -e "s/rv32imac/rv64imac/g" Makefile && \
	sed -i -e "s/ilp32/lp64/g" Makefile
RUN cd ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC  && \
	make clean && \
	make PICOLIBC=1 DEBUG=1 && \
	riscv64-unknown-elf-objcopy -O binary build/RTOSDemo.axf build/RTOSDemo.bin

RUN cd ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V_RV64_SiFive_HiFive-Premier-P550_GCC && \
	make clean && \
	make PICOLIBC=1 DEBUG=1 && \
	riscv64-unknown-elf-objcopy -O binary build/RTOSDemo.axf build/RTOSDemo.bin 

# Setup for Yocto
RUN apt update && \
	apt install -y gawk wget git diffstat unzip texinfo gcc build-essential chrpath socat cpio python3 python3-pip python3-pexpect xz-utils debianutils iputils-ping python3-git python3-jinja2 python3-subunit zstd liblz4-tool file locales libacl1 && \
	pip3 install kas --break-system-packages
RUN locale-gen en_US.UTF-8
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

RUN cd ${RISCV} && \
	mkdir yocto

# create a group/user
RUN groupadd --gid 2000 buildgroup

# create a non-root user
RUN useradd --home-dir ${RISCV}/yocto -s /bin/bash \
        --non-unique --uid 2000 --gid 2000 --groups sudo \
        yoctouser

# give users in the sudo group sudo access in the container
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

RUN chown -R yoctouser:buildgroup ${RISCV}/yocto

USER yoctouser

# Clone Yocto
RUN cd ${RISCV}/yocto && \
	git clone git://git.yoctoproject.org/poky && \
	cd poky && \
	git checkout origin/scarthgap -b my-scarthgap && \
	mkdir dist && cd dist && \
	git clone https://github.com/petitstrawberry/meta-sifive.git -b dev/meta-sifive/violetvm/hifive-premier-p550

RUN cd ${RISCV}/yocto/poky/dist/meta-sifive && \
	git checkout 4d2ff4702ac131385b90218d54b5e0a2abdccbaa

# Checkout and modify kas
RUN cd ${RISCV}/yocto/poky/dist && \
	kas checkout ./meta-sifive/scripts/kas/hifive-premier-p550.yml && \
	sed -i -e "s/INHERIT += \"sanity\"/# INHERIT += \"sanity\"/g" ./openembedded-core/meta/conf/sanity.conf

# Build by kas
RUN cd ${RISCV}/yocto/poky/dist && \
	kas build ./meta-sifive/scripts/kas/hifive-premier-p550.yml	

USER root

# === Debug for physical device ===
RUN apt update && \
	apt install -y minicom lsof openocd

# Build board firmware after the guest environment to reuse its Docker cache.
COPY violet_debug_bootloader/Cargo.toml violet_debug_bootloader/Cargo.lock /opt/riscv/violet-debug-source/violet_debug_bootloader/
COPY violet_debug_bootloader/src /opt/riscv/violet-debug-source/violet_debug_bootloader/src
COPY violet_debug_bootloader/lds /opt/riscv/violet-debug-source/violet_debug_bootloader/lds
COPY violet_debug_bootloader/.cargo /opt/riscv/violet-debug-source/violet_debug_bootloader/.cargo
COPY env/hifive_premier_p550/hifive_premier_p550.json /opt/riscv/violet-debug-source/env/hifive_premier_p550/
COPY rust-toolchain /opt/riscv/violet-debug-source/
COPY LICENSE.txt /opt/riscv/violet-debug-bootloader/LICENSE.txt
RUN cd /opt/riscv/violet-debug-source/violet_debug_bootloader && \
    cargo build --locked && \
    riscv64-unknown-elf-objcopy -O binary \
      target/hifive_premier_p550/debug/violet_debug_bootloader \
      /opt/riscv/violet-debug-bootloader/payload.bin
COPY --chown=2000:2000 env/hifive_premier_p550/meta-violet /opt/riscv/yocto/poky/dist/meta-violet
USER yoctouser
RUN cd ${RISCV}/yocto/poky/dist && kas build ./meta-violet/kas.yml
USER root
RUN install -m 644 \
    ${RISCV}/yocto/poky/dist/build/tmp-glibc/deploy/images/hifive-premier-p550/violet_debug_bootloader_ddr5_secboot.bin \
    ${RISCV}/violet-debug-bootloader/violet_debug_bootloader_ddr5_secboot.bin

WORKDIR /workspaces/Violet
