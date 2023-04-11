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
                li      t0, 0
                li      t1, 14
                sll     t0, t0, t1
                la      sp, __KERNEL_SP_BASE
                add     sp, sp, t0

                j       setup_cpu
        "
        :
        :
        :
        : "volatile");
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
                add     t0, a0, 0  // a0にコア番号が格納されている
                li      t1, 14
                sll     t0, t0, t1
                la      sp, __KERNEL_SP_BASE
                add     sp, sp, t0

                jalr    a1
        "
        :
        :
        :
        : "volatile");
    }
}

#[cfg(target_arch = "riscv64")]
#[export_name = "_start_trap"]
#[naked]
pub extern "C" fn _start_trap() {
    unsafe {
        asm! ("
        // from kernel
        .align 8
            csrrw sp, 0x140, sp // CSR=0x140=sscratch

            la sp, __KERNEL_SP_BASE //[todo fix]
            addi sp, sp, -32*8

            // Store registers
            sd   x0, 0*8(sp)
            sd   x1, 1*8(sp)
            sd   x2, 2*8(sp)
            sd   x3, 3*8(sp)
            sd   x4, 4*8(sp)
            sd   x5, 5*8(sp)
            sd   x6, 6*8(sp)
            sd   x7, 7*8(sp)
            sd   x8, 8*8(sp)
            sd   x9, 9*8(sp)
            sd   x10, 10*8(sp)
            sd   x11, 11*8(sp)
            sd   x12, 12*8(sp)
            sd   x13, 13*8(sp)
            sd   x14, 14*8(sp)
            sd   x15, 15*8(sp)
            sd   x16, 16*8(sp)
            sd   x17, 17*8(sp)
            sd   x18, 18*8(sp)
            sd   x19, 19*8(sp)
            sd   x20, 20*8(sp)            
            sd   x21, 21*8(sp)
            sd   x22, 22*8(sp)
            sd   x23, 23*8(sp)
            sd   x24, 24*8(sp)
            sd   x25, 25*8(sp)
            sd   x26, 26*8(sp)
            sd   x27, 27*8(sp)
            sd   x28, 28*8(sp)
            sd   x29, 29*8(sp)
            sd   x30, 30*8(sp)
            sd   x31, 31*8(sp)

            csrr t0, sepc
            sd   t0, 32*8(sp)

            addi a0, sp, 0
            jal ra, trap_handler

            ld   t0, 32*8(sp)
            csrw sepc, t0

            // Restore the registers from the stack.
            ld   x0, 0*8(sp)
            ld   x1, 1*8(sp)
            ld   x2, 2*8(sp)
            ld   x3, 3*8(sp)
            ld   x4, 4*8(sp)
            ld   x5, 5*8(sp)
            ld   x6, 6*8(sp)
            ld   x7, 7*8(sp)
            ld   x8, 8*8(sp)
            ld   x9, 9*8(sp)
            ld   x10, 10*8(sp)
            ld   x11, 11*8(sp)
            ld   x12, 12*8(sp)
            ld   x13, 13*8(sp)
            ld   x14, 14*8(sp)
            ld   x15, 15*8(sp)
            ld   x16, 16*8(sp)
            ld   x17, 17*8(sp)
            ld   x18, 18*8(sp)
            ld   x19, 19*8(sp)
            ld   x20, 20*8(sp)
            ld   x21, 21*8(sp)
            ld   x22, 22*8(sp)
            ld   x23, 23*8(sp)
            ld   x24, 24*8(sp)
            ld   x25, 25*8(sp)
            ld   x26, 26*8(sp)
            ld   x27, 27*8(sp)
            ld   x28, 28*8(sp)
            ld   x29, 29*8(sp)
            ld   x30, 30*8(sp)
            ld   x31, 31*8(sp)
            
            addi sp, sp, 32*8
            
            csrr sp, 0x140 // CSR=0x140=sscratch

            sret
        "
        :
        :
        :
        : "volatile");
    }
}
