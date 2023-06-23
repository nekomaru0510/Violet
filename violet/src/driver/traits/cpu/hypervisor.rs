//! Hypervisor(Virtualization) Extension Trait

use crate::driver::traits::cpu::context::TraitContext;

pub trait HypervisorT {
    type Context: TraitContext;
    fn init();
    fn hook(vecid: usize, func: fn(regs: *mut usize));
}
