#![no_std]

#![feature(llvm_asm)]
#![feature(naked_functions)]

//#[cfg(all(target_arch = "riscv64", target_os = "none"))]
#[link_section = ".reset.boot"]
#[export_name = "_start"]
#[naked]
pub extern "C" fn _start() {
    unsafe {
        llvm_asm! ("
            .align 4
            //set sp
            csrr    t0, mhartid
            la      t1, __STACK_SIFT
            sll     t0, t0, t1
            la      sp, __KERNEL_SP_BASE
            add     sp, sp, t0

            # AP wait for interrupt
            csrr    a0, mhartid
            bnez    a0, ap

            j       boot_init

    ap:
            wfi
            j       boot_init        
            "
        :
        :
        :
        : "volatile");
    }
}

/// 割込みハンドラ
#[no_mangle]
pub extern "C" fn interrupt_handler() {
    /*
    unsafe {
        let res = &mut Table::table().resource;
        res.io.timer.disable_interrupt();
        res.cpu.disable_interrupt();
    }
    vkth::int::entry();
    */
}

/// 割込みトラップのエントリポイント
#[export_name = "_start_trap"]
#[naked]
pub extern "C" fn _start_trap() {
    unsafe {
        llvm_asm! ("
        // from kernel
        .align 4
            csrrw sp, 0x340, sp // CSR=0x340=mscratch

            addi sp, sp, -16*4

            csrw mepc, ra

            // Store registers
            sw   ra, 0*4(sp)
            sw   t0, 1*4(sp)
            sw   t1, 2*4(sp)
            sw   t2, 3*4(sp)
            sw   t3, 4*4(sp)
            sw   t4, 5*4(sp)
            sw   t5, 6*4(sp)
            sw   t6, 7*4(sp)
            sw   a0, 8*4(sp)
            sw   a1, 9*4(sp)
            sw   a2, 10*4(sp)
            sw   a3, 11*4(sp)
            sw   a4, 12*4(sp)
            sw   a5, 13*4(sp)
            sw   a6, 14*4(sp)
            sw   a7, 15*4(sp)

            jal ra, interrupt_handler

            // Restore the registers from the stack.
            lw   ra, 0*4(sp)
            lw   t0, 1*4(sp)
            lw   t1, 2*4(sp)
            lw   t2, 3*4(sp)
            lw   t3, 4*4(sp)
            lw   t4, 5*4(sp)
            lw   t5, 6*4(sp)
            lw   t6, 7*4(sp)
            lw   a0, 8*4(sp)
            lw   a1, 9*4(sp)
            lw   a2, 10*4(sp)
            lw   a3, 11*4(sp)
            lw   a4, 12*4(sp)
            lw   a5, 13*4(sp)
            lw   a6, 14*4(sp)
            lw   a7, 15*4(sp)

            addi sp, sp, 16*4

            mret
        "
        :
        :
        :
        : "volatile");
    }
}