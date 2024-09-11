//! Virtual mtvec csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::extension::hypervisor::Hext;

pub struct Vmtvec {}

impl Vmtvec {
    pub fn new() -> Self {
        Vmtvec {}
    }
}

impl VirtualRegisterT for Vmtvec {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        Hext::set_vs_vector(val);
    }

    fn read(&mut self) -> u64 {
        Hext::get_vs_vector() as u64
    }
}

