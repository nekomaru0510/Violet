//! Traits for architecture specific processing

pub mod context;
pub mod hypervisor;
pub mod mmu;
pub mod registers;

// Processor Core specific processing
pub trait TraitCpu {
    fn setup(&self);
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
