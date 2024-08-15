use core::arch::asm;

#[cfg(target_arch = "riscv64")]
#[link_section = ".reset.boot"]
#[export_name = "_start"]
#[naked]
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        asm! ("
        .option norvc
        .align 8
                li      t0, 1        
                li      t1, 14
                sll     t0, t0, t1
                mul     t0, t0, a0          // mulを使うかは要検討
                la      sp, __KERNEL_SP_BOTTOM
                add     sp, sp, t0

                j       setup_cpu
        ",
        options(noreturn)
        );
    }
}

#[cfg(target_arch = "riscv64")]
#[export_name = "_start_ap"]
#[naked]
#[no_mangle]
pub extern "C" fn _start_ap() {
    unsafe {
        asm! ("
        .align 8
                li      t0, 1        
                li      t1, 14
                sll     t0, t0, t1
                mul     t0, t0, a0          // mulを使うかは要検討
                la      sp, __KERNEL_SP_BOTTOM
                add     sp, sp, t0

                jalr    a1
        ",
        options(noreturn)
        );
    }
}
