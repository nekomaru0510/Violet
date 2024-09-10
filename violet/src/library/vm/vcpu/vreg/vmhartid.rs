//! Virtual register returning a fixed value

use crate::library::vm::vcpu::vreg::VirtualRegisterT;

pub struct Vreg {
    val: u64,
}

impl VirtualRegisterT for Vreg {
    //type Regsize = u64;

    fn write(&mut self, val: u64) {
        ()
    }

    fn read(&mut self) -> u64 {
        self.val
    }
}

impl Vreg {
    pub fn new(vcpuid: u64) -> Self {
        Vreg { val: vcpuid }
    }
}
