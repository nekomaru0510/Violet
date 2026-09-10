//! Virtual mscratch csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;

pub struct Vmscratch {
    val: u64,
}

impl Vmscratch {
    pub fn new() -> Self {
        Vmscratch { val: 0 }
    }
}

impl VirtualRegisterT for Vmscratch {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        self.val = val;
    }

    fn read(&mut self) -> u64 {
        self.val
    }
}

