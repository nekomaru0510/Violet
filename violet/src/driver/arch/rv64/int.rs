//! RV64用 CPU内割込み機能モジュール

use super::csr::sip::*;
use super::csr::sie::*;
use super::csr::sstatus::*;

extern crate register;
use register::cpu::RegisterReadWrite;

#[derive(Clone)]
pub struct Rv64Int {
    pub sstatus: Sstatus,
    pub sip: Sip,
    pub sie: Sie,
}

impl Rv64Int {
    pub const fn new() -> Self {
        Rv64Int {
            sstatus: Sstatus {},
            sip: Sip {},
            sie: Sie {},
        }
    }

    /* supervisorモードの割込みを有効化 */
    pub fn enable_s(&self) {
        self.sstatus.modify(sstatus::SIE::SET);
    }

    /* supervisorモードの割込みを無効化 */
    pub fn disable_s(&self) {
        self.sstatus.modify(sstatus::SIE::CLEAR);
    }

    /* supervisorモードの指定割込みを有効化 */
    pub fn enable_mask_s(&self, int_mask: usize) {
        let sint_mask = 0x222 & int_mask; // sieの有効ビットでマスク
        self.sie.set(self.sie.get() | sint_mask as u64);
    }


    /* supervisorモードの指定割込みを無効化 */
    pub fn disable_mask_s(&self, int_mask: usize) {
        let sint_mask = 0x222 & int_mask; // sieの有効ビットでマスク
        self.sie.set(self.sie.get() & !(sint_mask as u64));
    }



}