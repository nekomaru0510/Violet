//! Virtual mideleg csr

//! Virtual medeleg csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;

pub struct Vmedeleg {
}

impl VirtualRegisterT for Vmedeleg {
    //type Regsize = u64;

    fn write(&mut self, _val: u64) {
        ()
    }

    // Hardwire to 0
    fn read(&mut self) -> u64 {
        0
    }
}

impl Vmedeleg {
    pub fn new() -> Self {
        Vmedeleg {}
    }
}
