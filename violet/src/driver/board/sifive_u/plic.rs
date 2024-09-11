//! Sifive Platform-Level Interrupt Controller(PLIC)

use core::ptr::{read_volatile, write_volatile};

use crate::driver::traits::intc::TraitIntc;

use crate::arch::traits::TraitArch;
use crate::environment::Arch;

#[derive(Clone)]
pub struct Plic {
    base: usize,
}

const INT_PRIO_SOURCE0: usize = 0;
const INT_ENABLE0_HART_OFFSET: usize = 0x100;
const INT_ENABLE0_CONTEXT0: usize = 0x2080;
const PRIO_THRESHOLD_HART_OFFSET: usize = 0x2000;
const PRIO_THRESHOLD_CONTEXT0: usize = 0x20_1000;
const PRIO_THRESHOLD_CONTEXT1: usize = PRIO_THRESHOLD_CONTEXT0 + PRIO_THRESHOLD_HART_OFFSET;
const CLAIM_COMPLETE_HART_OFFSET: usize = 0x2000;
const CLAIM_COMPLETE_CONTEXT0: usize = 0x20_1004;
const CLAIM_COMPLETE_CONTEXT1: usize = CLAIM_COMPLETE_CONTEXT0 + CLAIM_COMPLETE_HART_OFFSET;

impl TraitIntc for Plic {
    fn enable_interrupt(&self, id: u32) {
        self.set_enable(id);
    }

    fn disable_interrupt(&self, id: u32) {
        self.clear_enable(id);
    }

    // Get the interrupt number of the highest priority pending state
    fn get_pend_int(&self) -> u32 {
        self.get_claim_complete()
    }

    // Store the interrupt number that has been processed
    fn set_comp_int(&self, id: u32) {
        self.set_claim_complete(id);
    }

    fn set_prio(&self, id: u32, val: u32) {
        unsafe {
            write_volatile(
                (self.base + INT_PRIO_SOURCE0 + (id * 4) as usize) as *mut u32,
                val & 0x7,
            );
        }
    }

    fn set_priority_threshold(&self, val: u32) {
        unsafe {
            write_volatile(
                (self.base + PRIO_THRESHOLD_CONTEXT0 + PRIO_THRESHOLD_HART_OFFSET * Arch::get_cpuid())
                    as *mut u32,
                val,
            );
        }
    }
}

impl Plic {
    pub fn new(base: usize) -> Self {
        Plic { base }
    }

    pub fn set_enable(&self, id: u32) {
        let offset = ((id / 32) * 4) as usize/* + INT_ENABLE0_HART_OFFSET * get_cpuid()*/;
        let val = 0x01 << (id % 32) as u32;

        unsafe {
            write_volatile((self.base + INT_ENABLE0_CONTEXT0 + offset) as *mut u32, val);
        }
    }

    // [todo fix] Make it a clear process
    pub fn clear_enable(&self, id: u32) {
        let offset = ((id / 32) * 4) as usize/* + INT_ENABLE0_HART_OFFSET * get_cpuid()*/;
        let val = 0x01 << (id % 32) as u32;

        unsafe {
            write_volatile((self.base + INT_ENABLE0_CONTEXT0 + offset) as *mut u32, val);
        }
    }

    pub fn get_claim_complete(&self) -> u32 {
        unsafe {
            read_volatile(
                (self.base + CLAIM_COMPLETE_CONTEXT0 + CLAIM_COMPLETE_HART_OFFSET * Arch::get_cpuid())
                    as *const u32,
            )
        }
    }

    pub fn set_claim_complete(&self, id: u32) {
        unsafe {
            write_volatile(
                (self.base + CLAIM_COMPLETE_CONTEXT0 + CLAIM_COMPLETE_HART_OFFSET * Arch::get_cpuid())
                    as *mut u32,
                id,
            );
        }
    }
}
