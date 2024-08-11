//! RV64用 CPU内割込み機能モジュール

use crate::arch::rv64;
use rv64::csr::sie::*;
use rv64::csr::sstatus::*;

extern crate register;
use register::cpu::RegisterReadWrite;

/* 割込み */
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

    /* supervisorモードの割込みを有効化 */
    pub fn enable_s() {
        Sstatus.modify(sstatus::SIE::SET);
    }

    /* supervisorモードの割込みを無効化 */
    pub fn disable_s() {
        Sstatus.modify(sstatus::SIE::CLEAR);
    }

    /* supervisorモードの指定割込みを有効化 */
    pub fn enable_mask_s(int_mask: usize) {
        let sint_mask = 0x222 & int_mask; // sieの有効ビットでマスク
        Sie.set(Sie.get() | sint_mask as u64); // [todo fix]csrrs系命令を用いる
    }

    /* supervisorモードの指定割込みを無効化 */
    pub fn disable_mask_s(int_mask: usize) {
        let sint_mask = 0x222 & int_mask; // sieの有効ビットでマスク
        Sie.set(Sie.get() & !(sint_mask as u64));
    }
}
