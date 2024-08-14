//! Virtual mcause csr

extern crate violet;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::csr::vscause::Vscause;
use violet::arch::rv64::trap::TrapVector;

pub struct Vmcause {
    val: u64,
}

impl Vmcause {
    pub fn new() -> Self {
        Vmcause { val: 0 }
    }
}

impl VirtualRegisterT for Vmcause {
    //type Regsize = u64;
    
    fn write(&mut self, _val: u64) {
        //Vscause.set(val);
    }

    fn read(&mut self) -> u64 {

        self.val = Vscause::get();

        self.val = match self.val as usize {
            TrapVector::SUPERVISOR_SOFTWARE_INTERRUPT => TrapVector::MACHINE_SOFTWARE_INTERRUPT,
            TrapVector::SUPERVISOR_TIMER_INTERRUPT => TrapVector::MACHINE_TIMER_INTERRUPT,
            TrapVector::SUPERVISOR_EXTERNAL_INTERRUPT => TrapVector::MACHINE_EXTERNAL_INTERRUPT,
            TrapVector::ENVIRONMENT_CALL_FROM_VSMODE => TrapVector::ENVIRONMENT_CALL_FROM_MMODE,
            _ => self.val as usize,
        } as u64;
        self.val
    }
}

