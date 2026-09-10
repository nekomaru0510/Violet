do_deploy:append() {
    install -m 644 ${WORKDIR}/bootloader_ddr5_secboot.bin \
        ${DEPLOYDIR}/violet_debug_bootloader_ddr5_secboot.bin
}
