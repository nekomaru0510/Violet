//! RISC-V 汎用レジスタ

use crate::driver::traits::cpu::registers::TraitRegisters;

/* 割込み・例外元のコンテキストを示す */
#[derive(Clone, Copy)]
pub struct Registers {
    pub reg: [usize; 32],
    pub epc: usize,
}

pub const ZERO: usize = 0;
pub const RA: usize = 1;
pub const SP: usize = 2;
pub const GP: usize = 3;
pub const TP: usize = 4;
pub const T0: usize = 5;
pub const T1: usize = 6;
pub const T2: usize = 7;
pub const S0: usize = 8;
pub const FP: usize = 8;
pub const S1: usize = 9;
pub const A0: usize = 10;
pub const A1: usize = 11;
pub const A2: usize = 12;
pub const A3: usize = 13;
pub const A4: usize = 14;
pub const A5: usize = 15;
pub const A6: usize = 16;
pub const A7: usize = 17;
pub const S2: usize = 18;
pub const S3: usize = 19;
pub const S4: usize = 20;
pub const S5: usize = 21;
pub const S6: usize = 22;
pub const S7: usize = 23;
pub const S8: usize = 24;
pub const S9: usize = 25;
pub const S10: usize = 26;
pub const S11: usize = 27;
pub const T3: usize = 28;
pub const T4: usize = 29;
pub const T5: usize = 30;
pub const T6: usize = 31;

pub const NUM_OF_REGS: usize = 32;

impl Registers {
    pub fn new() -> Self {
        Registers {
            reg: [0; 32],
            epc: 0,
        }
    }

    pub fn from(sp: *mut usize) -> &'static mut Self {
        unsafe { &mut *(sp as *mut Self) }
    }
}

impl TraitRegisters for Registers {
    // レジスタの退避
    fn switch(&mut self, regs: &mut Self) {
        let tmp: Registers = *regs;
        *regs = *self;
        *self = tmp;
    }
}

#[test_case]
fn test_regs() -> Result<(), &'static str> {
    let mut r1 = Registers::new();
    let mut r2 = Registers::new();

    r1.reg[A0] = 1;

    r1.switch(&mut r2);

    if r1.reg[A0] != 0 {
        Err("Fail to switch registers")
    } else if r2.reg[A0] != 1 {
        Err("Fail to switch registers")
    } else {
        Ok(())
    }
}
