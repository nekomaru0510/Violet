//! RISC-V 汎用レジスタ

use crate::driver::traits::cpu::registers::TraitRegisters;

/* 割込み・例外元のコンテキストを示す */
#[derive(Clone, Copy)]
pub struct Registers {
    pub zero: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize, //fp
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub epc: usize,
}

/*
impl TraitRegisters for Registers {
    // レジスタの退避
    fn save_from(&mut self, from: &mut Self) {
        self = from;
    }

    // レジスタの復帰
    fn restore_to(&mut self, to: &mut Self) {
        to = self
    }
}
*/