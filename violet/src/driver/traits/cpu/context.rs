//! コンテキスト用トレイト

use super::registers::TraitRegisters;

pub trait TraitContext {
    type Registers: TraitRegisters;
    fn new() -> Self;
    fn switch(&mut self, regs: &mut Self::Registers);
    fn set(&mut self, idx: usize, value: usize);
    fn get(&self, idx: usize) -> usize;
    /* 格納されているコンテキストに移行する(直接ジャンプ) */
    fn jump(&self);
}
