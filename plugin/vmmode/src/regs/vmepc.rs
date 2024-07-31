//! Virtual mepc csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::driver::arch::rv64::csr::vsepc::Vsepc;
extern crate register;
use register::cpu::RegisterReadWrite;

pub struct Vmepc {
    val: u64,
}

impl Vmepc {
    pub fn new() -> Self {
        Vmepc { val: 0 }
    }
}

impl VirtualRegisterT for Vmepc {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        //Vsepc.set(self.val);
        Vsepc.set(val);
    }

    fn read(&mut self) -> u64 {
        self.val = Vsepc.get();
        self.val
    }
}

