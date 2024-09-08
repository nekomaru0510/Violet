//! Hypervisor(Virtualization) Extension Trait

use crate::arch::traits::context::TraitContext;

pub trait HypervisorT 
{
    type Context: TraitContext;
    fn init();
    fn hook(vecid: usize, func: fn(regs: *mut usize));
    fn mmu_enable();
    fn map_vaddr(vaddr: usize, paddr: usize, size: usize);
    // 仮想アドレスのパーミッション設定
    //fn set_attribute(vaddr: usize, paddr: usize, size: usize, attr: usize);
    // 仮想アドレス->物理アドレスへの変換
}
