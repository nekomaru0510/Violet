.option norvc
.section .reset.boot, "ax",@progbits
.global _start
_start:
        # set trap_vector
        la      t0, _start_trap
        csrw    mtvec, t0
        csrr    t1, mtvec

        # set sp
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

