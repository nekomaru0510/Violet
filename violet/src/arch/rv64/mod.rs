//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

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

use crate::system::init_system;
use crate::system::enter_container;
use crate::system::config;

use super::traits::TraitCpu;
use super::traits::TraitArch;

use boot::_start_ap;

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
    container_id: usize,
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

    fn get_core() -> &'static Self where Self: Sized{
        unsafe {
            let scratch: &Rv64 = transmute(Sscratch::get());
            if Sscratch::get() == 0 {
                panic!("CPU structure is not found.");
            } else {
                scratch
            }
        }
    }

    fn get_mut_core() -> &'static mut Self where Self: Sized{
        unsafe {
            let scratch: &'static mut Rv64 = transmute(Sscratch::get());
            if Sscratch::get() == 0 {
                panic!("CPU structure is not found.");
            } else {
                scratch
            }
        }
    }

    fn set_container_id(&mut self, id: usize) {
        self.container_id = id;
    }

    fn get_container_id(&self) -> usize {
        self.container_id
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
            container_id: 0,
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
pub extern "C" fn setup_boot(cpu_id: usize) {
    /* 
     * Rv64 structure is created on the stack. 
     * This stack isn't destroyed until finish hypervisor.
     * That's why we can use this structure before heap initialization.
     */
    let cpu = Rv64::new(cpu_id as u64);
    cpu.setup();
    set_container_id(cpu_id);

    wakeup_all_cpus(cpu_id);

    init_system(cpu_id);
}

type ExternFn = extern "C" fn(usize);

// Executed immediately after boot
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn setup_ap(cpu_id: usize, next: ExternFn) {
    /* 
     * Rv64 structure is created on the stack. 
     * This stack isn't destroyed until finish hypervisor.
     * That's why we can use this structure before heap initialization.
     */
    let cpu = Rv64::new(cpu_id as u64);
    cpu.setup();
    set_container_id(cpu_id);

    next(cpu_id);
}

fn wakeup_all_cpus(cpu_id: usize) {

    let num_of_cpus = config::get_num_of_cpus();
    for i in 0..num_of_cpus {
        if i as usize != cpu_id {
            sbi::sbi_hart_start(i as u64, _start_ap as u64, enter_container as u64);
        }
    }
}

fn set_container_id(cpu_id: usize) {
    let mut container_id = 0;

    // Get Container ID from System Configuration
    while (container_id) == 0 {
        container_id = config::get_container_id(cpu_id);
    };
    // Set Container ID
    Rv64::get_mut_core().set_container_id(container_id);
}

#[test_case]
fn test_rv64() -> Result<(), &'static str> {
    Ok(())
}
