//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Traits for architecture specific processing

pub mod context;
pub mod hypervisor;
pub mod mmu;
pub mod registers;

// Processor Core specific processing
pub trait TraitCpu {
    fn setup(&self);
    fn get_core() -> &'static Self where Self: Sized;
}

// Architecture specific processing
pub trait TraitArch {
    fn get_cpuid() -> usize;
    fn wakeup(cpuid: usize);
    fn sleep();
    fn enable_vector(vecid: usize) -> Result<(), ()>;
    fn register_vector(vecid: usize, func: fn(regs: *mut usize)) -> Result<(), ()>;
    fn call_vector(vecid: usize, regs: *mut usize) -> Result<(), ()>;
    fn enable_interrupt();
    fn disable_interrupt();
    fn ipi(core_id: usize);
}
