//! Hypervisor(Virtualization) Extension Trait

use crate::arch::traits::context::TraitContext;

pub trait HypervisorT {
    type Context: TraitContext;
    fn init();
    fn hook(vecid: usize, func: fn(regs: *mut usize));
    fn mmu_enable();
}
