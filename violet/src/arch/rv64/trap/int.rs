//! Risc-V Interrupt

use crate::arch::rv64;
use rv64::csr::sie::*;
use rv64::csr::sstatus::*;

#[derive(Clone, Copy)]
pub struct Interrupt();

impl Interrupt {
    pub const SUPERVISOR_SOFTWARE_INTERRUPT: usize = 1;
    pub const VIRTUAL_SUPERVISOR_SOFTWARE_INTERRUPT: usize = 2;
    pub const MACHINE_SOFTWARE_INTERRUPT: usize = 3;
    pub const SUPERVISOR_TIMER_INTERRUPT: usize = 5;
    pub const VIRTUAL_SUPERVISOR_TIMER_INTERRUPT: usize = 6;
    pub const MACHINE_TIMER_INTERRUPT: usize = 7;
    pub const SUPERVISOR_EXTERNAL_INTERRUPT: usize = 9;
    pub const VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT: usize = 10;
    pub const MACHINE_EXTERNAL_INTERRUPT: usize = 11;
    pub const SUPERVISOR_GUEST_EXTERNAL_INTERRUPT: usize = 12;

    pub fn bit(val: usize) -> usize {
        1 << val
    }

    // Enable S-mode interrupt
    pub fn enable_s() {
        Sstatus::write(SIE, SIE::SET);
    }

    // Disable S-mode interrupt
    pub fn disable_s() {
        Sstatus::write(SIE, SIE::CLEAR);
    }

    // Enable specified interrupt in S-mode
    pub fn enable_mask_s(int_mask: usize) {
        let sint_mask = 0x222 & int_mask;
        Sie::set(Sie::get() | sint_mask as u64);
    }

    // Disable specified interrupt in S-mode
    pub fn disable_mask_s(int_mask: usize) {
        let sint_mask = 0x222 & int_mask;
        Sie::set(Sie::get() & !(sint_mask as u64));
    }
}
