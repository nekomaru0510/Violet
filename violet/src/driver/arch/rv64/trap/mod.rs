//! 例外・割込みのトラップ
pub mod exc;
pub mod int;

extern crate register;
use register::cpu::RegisterReadWrite;

use super::csr::scause::*;
use super::trap::exc::Exception;
use super::trap::int::Interrupt;
use crate::environment::cpu;

/* 割込み・例外ベクタ */
pub struct TrapVector {
    handler: TrapHandler,
}

impl TrapVector {
    pub const INTERRUPT_OFFSET: usize = 0x8000_0000_0000_0000;

    pub const SUPERVISOR_SOFTWARE_INTERRUPT: usize =
        Interrupt::SUPERVISOR_SOFTWARE_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const VIRTUAL_SUPERVISOR_SOFTWARE_INTERRUPT: usize =
        Interrupt::VIRTUAL_SUPERVISOR_SOFTWARE_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const MACHINE_SOFTWARE_INTERRUPT: usize =
        Interrupt::MACHINE_SOFTWARE_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const SUPERVISOR_TIMER_INTERRUPT: usize =
        Interrupt::SUPERVISOR_TIMER_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const VIRTUAL_SUPERVISOR_TIMER_INTERRUPT: usize =
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const MACHINE_TIMER_INTERRUPT: usize =
        Interrupt::MACHINE_TIMER_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const SUPERVISOR_EXTERNAL_INTERRUPT: usize =
        Interrupt::SUPERVISOR_EXTERNAL_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT: usize =
        Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const MACHINE_EXTERNAL_INTERRUPT: usize =
        Interrupt::MACHINE_EXTERNAL_INTERRUPT + Self::INTERRUPT_OFFSET;
    pub const SUPERVISOR_GUEST_EXTERNAL_INTERRUPT: usize =
        Interrupt::SUPERVISOR_GUEST_EXTERNAL_INTERRUPT + Self::INTERRUPT_OFFSET;

    pub const INSTRUCTION_ADDRESS_MISALIGNED: usize = Exception::INSTRUCTION_ADDRESS_MISALIGNED;
    pub const INSTRUCTION_ACCESS_FAULT: usize = Exception::INSTRUCTION_ACCESS_FAULT;
    pub const ILLEGAL_INSTRUCTION: usize = Exception::ILLEGAL_INSTRUCTION;
    pub const BREAKPOINT: usize = Exception::BREAKPOINT;
    pub const LOAD_ADDRESS_MISALIGNED: usize = Exception::LOAD_ADDRESS_MISALIGNED;
    pub const LOAD_ACCESS_FAULT: usize = Exception::LOAD_ACCESS_FAULT;
    pub const STORE_AMO_ADDRESS_MISALIGNED: usize = Exception::STORE_AMO_ADDRESS_MISALIGNED;
    pub const STORE_AMO_ACCESS_FAULT: usize = Exception::STORE_AMO_ACCESS_FAULT;
    pub const ENVIRONMENT_CALL_FROM_UMODE_OR_VUMODE: usize =
        Exception::ENVIRONMENT_CALL_FROM_UMODE_OR_VUMODE;
    pub const ENVIRONMENT_CALL_FROM_HSMODE: usize = Exception::ENVIRONMENT_CALL_FROM_HSMODE;
    pub const ENVIRONMENT_CALL_FROM_VSMODE: usize = Exception::ENVIRONMENT_CALL_FROM_VSMODE;
    pub const ENVIRONMENT_CALL_FROM_MMODE: usize = Exception::ENVIRONMENT_CALL_FROM_MMODE;
    pub const INSTRUCTION_PAGE_FAULT: usize = Exception::INSTRUCTION_PAGE_FAULT;
    pub const LOAD_PAGE_FAULT: usize = Exception::LOAD_PAGE_FAULT;
    pub const STORE_AMO_PAGE_FAULT: usize = Exception::STORE_AMO_PAGE_FAULT;
    pub const INSTRUCTION_GUEST_PAGE_FAULT: usize = Exception::INSTRUCTION_GUEST_PAGE_FAULT;
    pub const LOAD_GUEST_PAGE_FAULT: usize = Exception::LOAD_GUEST_PAGE_FAULT;
    pub const VIRTUAL_INSTRUCTION: usize = Exception::VIRTUAL_INSTRUCTION;
    pub const STORE_AMO_GUEST_PAGE_FAULT: usize = Exception::STORE_AMO_GUEST_PAGE_FAULT;

    pub const fn new() -> Self {
        TrapVector {
            handler: TrapHandler::new(),
        }
    }

    pub fn register_vector(&mut self, vecid: usize, func: fn(regs: *mut usize)) {
        /* Interrupt */
        if vecid > Self::INTERRUPT_OFFSET {
            self.handler
                .register_interrupt(vecid - Self::INTERRUPT_OFFSET, func);
        }
        /* Exception */
        else {
            self.handler.register_exception(vecid, func);
        }
    }

    pub fn call_vector(&self, vecid: usize, regs: *mut usize) {
        /* Interrupt */
        if vecid > Self::INTERRUPT_OFFSET {
            self.handler
                .call_interrupt_handler(vecid - Self::INTERRUPT_OFFSET, regs);
        }
        /* Exception */
        else {
            self.handler.call_exception_handler(vecid, regs);
        }
    }
}

const NUM_OF_INTERRUPTS: usize = 32;
const NUM_OF_EXCEPTIONS: usize = 32;

pub struct TrapHandler {
    interrupt_handler: [Option<fn(regs: *mut usize)>; NUM_OF_INTERRUPTS],
    exception_handler: [Option<fn(regs: *mut usize)>; NUM_OF_EXCEPTIONS],
}

impl TrapHandler {
    pub const fn new() -> Self {
        TrapHandler {
            interrupt_handler: [None; NUM_OF_INTERRUPTS],
            exception_handler: [None; NUM_OF_EXCEPTIONS],
        }
    }

    pub fn register_interrupt(&mut self, int_num: usize, func: fn(regs: *mut usize)) {
        self.interrupt_handler[int_num] = Some(func);
    }

    pub fn register_exception(&mut self, exc_num: usize, func: fn(regs: *mut usize)) {
        self.exception_handler[exc_num] = Some(func);
    }

    pub fn call_interrupt_handler(&self, int_num: usize, regs: *mut usize) {
        match self.interrupt_handler[int_num] {
            None => (),
            Some(func) => func(regs),
        }
    }

    pub fn call_exception_handler(&self, exc_num: usize, regs: *mut usize) {
        match self.exception_handler[exc_num] {
            None => (),
            Some(func) => func(regs),
        }
    }
}

// 割込み・例外ハンドラ
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn trap_handler(regs: *mut usize) {
    /* 割込み・例外要因 */
    let scause = Scause {};
    //cpu().trap.call_vector(scause.get() as usize, regs);
    cpu().call_vector(scause.get() as usize, regs)
}

#[cfg(target_arch = "riscv64")]
#[export_name = "_start_trap"]
#[naked]
pub extern "C" fn _start_trap() {
    unsafe {
        asm! ("
        // from kernel
        .align 8

            // SPの退避
            csrrw tp, sscratch, tp
            sd  sp, 8(tp)
            sd  t0, 16(tp)
            
            // スタックサイズをspに格納
            li  t0, 1
            ld  sp, 24(tp)
            // コア番号の取得
            ld  t0, 0(tp)
            mul t0, t0, sp          // mulを使うかは要検討
            // SPの設定
            la  sp, __KERNEL_TRAP_SP_BOTTOM
            sub sp, sp, t0
            
            // t0の復帰
            ld  t0, 16(tp)

            addi sp, sp, -32*8

            // Store registers
            sd   x0, 0*8(sp)
            sd   x1, 1*8(sp)
            //sd   x2, 2*8(sp) /* sp */
            sd   x3, 3*8(sp)
            //sd   x4, 4*8(sp) /* tp */
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

            // spの復帰・格納
            ld   t0, 8(tp)
            sd   t0, 2*8(sp)

            // tpの復帰・sscratchの復帰
            csrrw tp, sscratch, tp
            sd   tp, 4*8(sp) /* tp */

            csrr t0, sepc
            sd   t0, 32*8(sp)

            addi a0, sp, 0
            jal ra, trap_handler

            ld   t0, 32*8(sp)
            csrw sepc, t0

            // Restore the registers from the stack.
            ld   x0, 0*8(sp)
            ld   x1, 1*8(sp)
            //ld   x2, 2*8(sp) /* sp */
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
            
            ld   x2, 2*8(sp) /* sp */
            //addi sp, sp, 32*8

            sret
        "
        :
        :
        :
        : "volatile");
    }
}
