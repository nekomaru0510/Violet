#[cfg(target_arch = "riscv64")]
#[link_section = ".reset.boot"]
#[export_name = "_start"]
#[naked]
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        asm! ("
        .option norvc
        //.section .reset.boot, \"ax\",@progbits
        .align 8
                // set sp
                // csrr    t0, mhartid
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
/*
#[cfg(target_arch = "riscv64")]
#[export_name = "_start_trap"]
#[naked]
pub extern "C" fn _start_trap() {
    unsafe {
        asm! ("
        // from kernel
        .align 8
            csrrw sp, 0x340, sp // CSR=0x340=mscratch

            addi sp, sp, -16*4

            csrw mepc, ra

            // Store registers
            sd   ra, 0*4(sp)
            sd   t0, 1*4(sp)
            sd   t1, 2*4(sp)
            sd   t2, 3*4(sp)
            sd   t3, 4*4(sp)
            sd   t4, 5*4(sp)
            sd   t5, 6*4(sp)
            sd   t6, 7*4(sp)
            sd   a0, 8*4(sp)
            sd   a1, 9*4(sp)
            sd   a2, 10*4(sp)
            sd   a3, 11*4(sp)
            sd   a4, 12*4(sp)
            sd   a5, 13*4(sp)
            sd   a6, 14*4(sp)
            sd   a7, 15*4(sp)

            addi a0, sp, 0
            jal ra, get_context

            // Restore the registers from the stack.
            ld   ra, 0*4(sp)
            ld   t0, 1*4(sp)
            ld   t1, 2*4(sp)
            ld   t2, 3*4(sp)
            ld   t3, 4*4(sp)
            ld   t4, 5*4(sp)
            ld   t5, 6*4(sp)
            ld   t6, 7*4(sp)
            ld   a0, 8*4(sp)
            ld   a1, 9*4(sp)
            ld   a2, 10*4(sp)
            ld   a3, 11*4(sp)
            ld   a4, 12*4(sp)
            ld   a5, 13*4(sp)
            ld   a6, 14*4(sp)
            ld   a7, 15*4(sp)

            addi sp, sp, 16*4

            mret
        "
        :
        :
        :
        : "volatile");
    }
}
*/
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
            addi sp, sp, -16*8

            //csrw sepc, ra

            // Store registers
            sd   ra, 0*8(sp)
            sd   t0, 1*8(sp)
            sd   t1, 2*8(sp)
            sd   t2, 3*8(sp)
            sd   t3, 4*8(sp)
            sd   t4, 5*8(sp)
            sd   t5, 6*8(sp)
            sd   t6, 7*8(sp)
            sd   a0, 8*8(sp)
            sd   a1, 9*8(sp)
            sd   a2, 10*8(sp)
            sd   a3, 11*8(sp)
            sd   a4, 12*8(sp)
            sd   a5, 13*8(sp)
            sd   a6, 14*8(sp)
            sd   a7, 15*8(sp)

            csrr t0, sepc
            sd   t0, 16*8(sp)
            csrr t0, scause
            sd   t0, 17*8(sp)
            csrr t0, stval
            sd   t0, 18*8(sp)
            csrr t0, sscratch
            sd   t0, 19*8(sp)

            addi a0, sp, 0
            jal ra, get_context

            ld   t0, 16*8(sp)
            csrw sepc, t0

            // Restore the registers from the stack.
            ld   ra, 0*8(sp)
            ld   t0, 1*8(sp)
            ld   t1, 2*8(sp)
            ld   t2, 3*8(sp)
            ld   t3, 4*8(sp)
            ld   t4, 5*8(sp)
            ld   t5, 6*8(sp)
            ld   t6, 7*8(sp)
            ld   a0, 8*8(sp)
            ld   a1, 9*8(sp)
            ld   a2, 10*8(sp)
            ld   a3, 11*8(sp)
            ld   a4, 12*8(sp)
            ld   a5, 13*8(sp)
            ld   a6, 14*8(sp)
            ld   a7, 15*8(sp)

            addi sp, sp, 16*8
            
            csrr sp, 0x140 // CSR=0x140=sscratch

            sret
        "
        :
        :
        :
        : "volatile");
    }
}