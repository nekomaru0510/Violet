//! 例外・割込みのトラップ
extern crate register;
use register::cpu::RegisterReadWrite;

use super::csr::scause::*;
use super::regs::Registers;
use super::{Interrupt, Exception};

const NUM_OF_INTERRUPTS: usize = 32;
const NUM_OF_EXCEPTIONS: usize = 32;

pub static mut INTERRUPT_HANDLER: [Option<fn(regs: &mut Registers)>; NUM_OF_INTERRUPTS] =
    [None; NUM_OF_INTERRUPTS];
pub static mut EXCEPTION_HANDLER: [Option<fn(regs: &mut Registers)>; NUM_OF_EXCEPTIONS] =
    [None; NUM_OF_EXCEPTIONS];

pub struct TrapHandler {
    interrupt_handler: [Option<fn(regs: &mut Registers)>; NUM_OF_INTERRUPTS],
    exception_handler: [Option<fn(regs: &mut Registers)>; NUM_OF_EXCEPTIONS],
}

impl TrapHandler {
    pub const fn new() -> Self {
        TrapHandler {
            interrupt_handler: [None; NUM_OF_INTERRUPTS],
            exception_handler: [None; NUM_OF_EXCEPTIONS],
        }
    }

    pub fn register_interrupt(&mut self, int_num: Interrupt, func: fn(regs: &mut Registers)) {
        self.interrupt_handler[int_num as usize] = Some(func);
    }

    pub fn register_exception(&mut self, exc_num: Exception, func: fn(regs: &mut Registers)) {
        self.exception_handler[exc_num as usize] = Some(func);
    }
}

// 割込み・例外ハンドラ
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn trap_handler(regs: &mut Registers) {
    /* 割込み・例外要因 */
    let scause = Scause {};
    let e: usize = scause.read(scause::EXCEPTION) as usize;
    let i: usize = scause.read(scause::INTERRUPT) as usize;

    /* 割込み・例外ハンドラの呼出し */
    unsafe {
        match i {
            0 => match EXCEPTION_HANDLER[e] {
                Some(func) => func(regs),
                None => (),
            },
            1 => match INTERRUPT_HANDLER[e] {
                Some(func) => func(regs),
                None => (),
            },
            _ => (),
        };
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

