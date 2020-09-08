//! cpu

#![no_std]

extern crate alloc;
use alloc::string::String;

use processor::Processor;

pub struct Cpu {
    pub processor: Processor,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {processor: Processor::new(0, String::from("imac")),}
    }
    
    pub fn enable_interrupt(&self) {
        self.processor.enable_interrupt();
    }

    pub fn disable_interrupt(&self) {
        self.processor.disable_interrupt();
    }
    
}
