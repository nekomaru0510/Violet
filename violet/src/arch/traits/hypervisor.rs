//! Hypervisor(Virtualization) Extension Trait

use crate::arch::traits::context::TraitContext;
use crate::arch::rv64::regs::Registers;

pub trait HypervisorT 
{
    type Context: TraitContext;
    fn init();
    fn reset();
    fn hook(vecid: usize, func: fn(regs: *mut usize));
    fn mmu_enable();
    fn map_vaddr(vaddr: usize, paddr: usize, size: usize);
    //fn set_attribute(vaddr: usize, paddr: usize, size: usize, attr: usize);
    fn v2p(vaddr: usize) -> usize;
    fn redirect_to_guest(regs: &mut Registers);
}
