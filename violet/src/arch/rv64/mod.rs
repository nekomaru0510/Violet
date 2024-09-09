//! RV64 CPU module

pub mod boot;
pub mod csr;
pub mod extension;
pub mod instruction;
pub mod mmu;
pub mod regs;
pub mod sbi;
pub mod trap;
pub mod vscontext;

use crate::kernel::boot_init;

use super::traits::TraitCpu;
use super::traits::TraitArch;

use instruction::Instruction;
use trap::TrapVector;
use trap::_start_trap;
use trap::int::Interrupt;

use core::intrinsics::transmute;

use csr::hstatus;
use csr::hstatus::*;
use csr::sscratch::Sscratch;
use csr::sstatus;
use csr::sstatus::*;
use csr::stvec::Stvec;

#[derive(Clone, Copy)]
pub enum PrivilegeMode {
    ModeM,
    ModeHS,
    ModeS,
    ModeHU,
    ModeU,
    ModeVS,
    ModeVU,
}

pub enum PagingMode {
    Bare = 0,
    Sv39x4 = 8,
    Sv48x4 = 9,
    Sv57x4 = 10,
}

pub struct Rv64 {
    cpu_id: u64,
    sp: usize,
    tmp0: usize,
    status: CpuStatus,
    trap: TrapVector,
}

#[derive(Copy, Clone)]
pub enum CpuStatus {
    STOPPED = 0x00,
    STARTED,       
    SUSPENDED,     
}

impl TraitCpu for Rv64 {
    fn setup(&self) {
        self.set_sscratch();
        self.set_default_vector();
        Rv64::enable_interrupt();
    }
}

impl TraitArch for Rv64 {
    fn wakeup(cpuid: usize) {
        sbi::sbi_hart_start(cpuid as u64, boot::_start_ap as u64, 0xabcd);
    }

    fn sleep() {
        sbi::sbi_hart_stop();
    }

    fn get_cpuid() -> usize {
        unsafe {
            let scratch: &Rv64 = transmute(Sscratch::get());
            if Sscratch::get() == 0 {
                0
            } else {
                scratch.cpu_id as usize
            }
        }
    }

    fn enable_vector(vecid: usize) -> Result<(), ()> {
        unsafe {
            let scratch: &mut Rv64 = transmute(Sscratch::get());
            if Sscratch::get() == 0 {
                Err(())
            } else {
                scratch.trap.enable_vector(vecid);
                Ok(())
            }
        }
    }

    fn register_vector(vecid: usize, func: fn(regs: *mut usize)) -> Result<(), ()> {
        unsafe {
            let scratch: &mut Rv64 = transmute(Sscratch::get());
            if Sscratch::get() == 0 {
                Err(())
            } else {
                scratch.trap.register_vector(vecid, func);
                Ok(())
            }
        }
    }

    fn call_vector(vecid: usize, regs: *mut usize) -> Result<(), ()> {
        unsafe {
            let scratch: &Rv64 = transmute(Sscratch::get());
            if Sscratch::get() == 0 {
                Err(())
            } else {
                scratch.trap.call_vector(vecid, regs);
                Ok(())
            }
        }
    }

    fn enable_interrupt() {
        Interrupt::enable_s();
    }

    fn disable_interrupt() {
        Interrupt::disable_s();
    }

    fn ipi(core_id: usize) {
        let hart_mask: u64 = 0x01 << core_id;
        sbi::sbi_send_ipi(&hart_mask);
    }
}

impl Rv64 {
    pub const fn new(id: u64) -> Self {
        Rv64 {
            cpu_id: id,
            sp: 0x0,
            tmp0: 0x0,
            status: CpuStatus::STARTED,
            trap: TrapVector::new(),
        }
    }

    pub fn set_sscratch(&self) {
        Sscratch::set(unsafe { transmute(self) });
    }

    pub fn set_default_vector(&self) {
        self.set_vector(_start_trap as usize);
    }

    fn set_vector(&self, addr: usize) {
        Stvec::set(addr as u64);
    }

    pub fn switch_hs_mode() {
        // Next mode is HS-mode
        Self::set_next_mode(PrivilegeMode::ModeHS);
        // switch next mode
        Instruction::sret(0, 0, 0);
    }

    pub fn set_next_mode(mode: PrivilegeMode) {
        match mode {
            PrivilegeMode::ModeS => {
                Sstatus::write(sstatus::SPP, sstatus::SPP::SET);
                Hstatus::write(hstatus::SPV, hstatus::SPV::CLEAR);
            }
            PrivilegeMode::ModeVS => {
                Sstatus::write(sstatus::SPP, sstatus::SPP::SET);
                Hstatus::write(hstatus::SPV, hstatus::SPV::SET);
                Hstatus::write(hstatus::SPV, hstatus::SPVP::SET);
            }
            PrivilegeMode::ModeHS => {
                Sstatus::write(sstatus::SPP, sstatus::SPP::SET);
                Hstatus::write(hstatus::SPV, hstatus::SPV::CLEAR);
            }
            _ => (),
        };
    }
}

// Executed immediately after boot
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn setup_cpu(cpu_id: usize) {
    boot_init(cpu_id);
}

#[test_case]
fn test_rv64() -> Result<(), &'static str> {
    Ok(())
}
