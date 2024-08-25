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
                /* a0 ... hartid */
                li      t0, 1        
                li      t1, 14
                sll     t0, t0, t1
                // [todo fix] mul instruction is not wanted, 
                // but if only shift operation is used, 
                // sp will be broken by optimization
                mul     t0, t0, a0          
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
        .align 8
                /* a0 ... hartid, a1 ... next function */
                li      t0, 1        
                li      t1, 14
                sll     t0, t0, t1
                // [todo fix] mul instruction is not wanted, 
                // but if only shift operation is used, 
                // sp will be broken by optimization
                mul     t0, t0, a0
                la      sp, __KERNEL_SP_BOTTOM
                add     sp, sp, t0

                jalr    a1
        ",
        options(noreturn)
        );
    }
}
