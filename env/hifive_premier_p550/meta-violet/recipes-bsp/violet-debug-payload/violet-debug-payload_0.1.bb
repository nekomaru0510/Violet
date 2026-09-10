SUMMARY = "Violet debug payload compiled by the Dockerfile"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://LICENSE.txt;md5=2f6f4be77591587201cd637133011d41"
FILESEXTRAPATHS:prepend := "/opt/riscv/violet-debug-bootloader:"
SRC_URI = "file://payload.bin file://LICENSE.txt"
S = "${WORKDIR}"
inherit deploy
INHIBIT_DEFAULT_DEPS = "1"
do_configure[noexec] = "1"
do_compile[noexec] = "1"
do_install[noexec] = "1"
do_deploy() {
    install -m 644 ${WORKDIR}/payload.bin ${DEPLOYDIR}/violet-debug-payload.bin
}
addtask deploy after do_install before do_build
COMPATIBLE_MACHINE = "hifive-premier-p550"
