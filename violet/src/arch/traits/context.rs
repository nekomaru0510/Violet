//! Context trait

use super::registers::TraitRegisters;

pub trait TraitContext {
    type Registers: TraitRegisters;
    fn new() -> Self;
    fn switch(&mut self, regs: &mut Self::Registers);
    fn set(&mut self, idx: usize, value: usize);
    fn get(&self, idx: usize) -> usize;
    // Jump to the context stored
    fn jump(&self);
}
