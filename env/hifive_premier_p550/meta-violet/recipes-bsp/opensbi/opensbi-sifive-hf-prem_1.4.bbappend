RISCV_SBI_PAYLOAD = "violet-debug-payload.bin"
unset RISCV_SBI_FDT
DEPENDS += "dtc-native"
do_compile[depends] += "violet-debug-payload:do_deploy virtual/bootloader:do_deploy"
do_prepare_host_dtb[depends] += "virtual/bootloader:do_deploy dtc-native:do_populate_sysroot"
EXTRA_OEMAKE:append = " FW_FDT_PATH=${WORKDIR}/violet-host.dtb"

# Change the host PMU map without modifying the guest's u-boot.dtb.
python do_prepare_host_dtb() {
    import os
    import shutil
    import subprocess

    tools = d.getVar('STAGING_BINDIR_NATIVE')
    temporary = os.path.join(d.getVar('WORKDIR'), 'host-packed.dtb')
    output = os.path.join(d.getVar('WORKDIR'), 'violet-host.dtb')
    shutil.copyfile(os.path.join(d.getVar('DEPLOY_DIR_IMAGE'), 'u-boot.dtb'), temporary)
    prop = 'riscv,event-to-mhpmevent'
    values = subprocess.check_output([os.path.join(tools, 'fdtget'), '-t', 'x',
                                     temporary, '/pmu', prop], text=True).split()
    cells = [int(value, 16) for value in values]
    if len(cells) % 3:
        bb.fatal('Unexpected PMU event map')
    mappings = {4: 0x0202, 6: 0x2001}
    for offset in range(0, len(cells), 3):
        if cells[offset] in mappings:
            cells[offset + 2] = mappings.pop(cells[offset])
    if mappings:
        bb.fatal('Required PMU events are missing')
    subprocess.check_call([os.path.join(tools, 'fdtput'), '-t', 'x', temporary,
                           '/pmu', prop] + [format(value, 'x') for value in cells])
    # Restore U-Boot's reserve slots and padding after fdtput packs the DTB.
    subprocess.check_call([os.path.join(tools, 'dtc'), '-I', 'dtb', '-O', 'dtb',
                           '-b', '0', '-@', '-R', '4', '-p', '0x1000',
                           '-o', output, temporary])
}
addtask prepare_host_dtb after do_prepare_recipe_sysroot before do_compile
