//! Boot code for RISC-V 64-bit architecture
use core::arch::asm;

/// Entry point called at violet startup
#[cfg(target_arch = "riscv64")]
#[link_section = ".reset.boot"]
#[export_name = "_start"]
#[naked]
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        asm! ("
        .option norvc
        .option norelax
        .align 8
                /* a0 = hartid */
                slli    a0, a0, 14                
                la      sp, __KERNEL_SP_BOTTOM
                add     sp, sp, t0

                j       setup_cpu
        ",
        options(noreturn)
        );
    }
}

/// Entry point for BSP
#[cfg(target_arch = "riscv64")]
#[export_name = "_start_ap"]
#[naked]
#[no_mangle]
pub extern "C" fn _start_ap() {
    unsafe {
        asm! ("
        .option norvc
        .option norelax
        .align 8
                /* a0 = hartid, a1 = next function*/
                slli    a0, a0, 14
                la      sp, __KERNEL_SP_BOTTOM
                add     sp, sp, t0

                jalr    a1
        ",
        options(noreturn)
        );
    }
}
