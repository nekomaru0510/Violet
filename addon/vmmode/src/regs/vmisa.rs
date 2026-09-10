//! Virtual misa csr


extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;

pub struct Vmisa {
    val: u64,
}

impl Vmisa {
    pub fn new() -> Self {
        Vmisa { val: 0x800000000010112d }
    }
}

impl VirtualRegisterT for Vmisa {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        self.val = val;
    }

    fn read(&mut self) -> u64 {
        self.val
    }
}

