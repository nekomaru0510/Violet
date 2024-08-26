//! Virtual mhartid csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;

pub struct Vmhartid {
    val: u64,
}

impl VirtualRegisterT for Vmhartid {
    //type Regsize = u64;

    fn write(&mut self, _val: u64) {
        ()
    }

    fn read(&mut self) -> u64 {
        self.val
    }
}

impl Vmhartid {
    pub fn new(vcpuid: u64) -> Self {
        Vmhartid { val: vcpuid }
    }
}
