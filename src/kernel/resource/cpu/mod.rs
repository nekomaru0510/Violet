use crate::kernel::driver::arch::rv32::RV32;

extern crate alloc;
use alloc::string::String;

pub struct Cpu {
    pub core: RV32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {core: RV32::new(0, String::from("imac")),}
    }

    pub fn enable_interrupt(&self) {
        self.core.enable_interrupt();
    }

    pub fn disable_interrupt(&self) {
        self.core.disable_interrupt();
    }
    
}
