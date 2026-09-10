//! Virtual mtval csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::csr::vstval::Vstval;
use violet::arch::rv64::trap::TrapVector;

pub struct Vmtval {
    val: u64,
}

impl Vmtval {
    pub fn new() -> Self {
        Vmtval { val: 0 }
    }
}

impl VirtualRegisterT for Vmtval {
    //type Regsize = u64;
    
    fn write(&mut self, _val: u64) {
        Vstval::set(_val);
    }

    fn read(&mut self) -> u64 {
        Vstval::get()
    }
}

