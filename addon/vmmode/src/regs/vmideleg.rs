//! Virtual mideleg csr

//! Virtual mhartid csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;

pub struct Vmideleg {
}

impl VirtualRegisterT for Vmideleg {
    //type Regsize = u64;

    fn write(&mut self, _val: u64) {
        ()
    }

    // Hardwire to 0
    fn read(&mut self) -> u64 {
        0
    }
}

impl Vmideleg {
    pub fn new() -> Self {
        Vmideleg {}
    }
}
