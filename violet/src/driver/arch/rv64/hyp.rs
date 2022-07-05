//! ハイパーバイザ拡張

extern crate register;
use register::cpu::RegisterReadWrite;

use crate::driver::traits::arch::riscv::PagingMode;

use super::csr::hcounteren::*;
use super::csr::hedeleg::*;
use super::csr::hgatp::*;
use super::csr::hgeie::*;
use super::csr::hideleg::*;
use super::csr::hie::*;
use super::csr::hstatus::*;
use super::csr::htval::*;
use super::csr::hvip::*;
use super::csr::vsatp::*;
use super::csr::vstval::*;

#[derive(Clone)]
pub struct Rv64Hyp {
    pub hstatus: Hstatus,
    pub hie: Hie,
    pub hvip: Hvip,
    pub hgatp: Hgatp,
    pub hgeie: Hgeie,
    pub htval: Htval,
    pub hideleg: Hideleg,
    pub hedeleg: Hedeleg,
    pub hcounteren: Hcounteren,
    pub vsatp: Vsatp,
    pub vstval: Vstval,
}

impl Rv64Hyp {
    pub const fn new() -> Self {
        Rv64Hyp {
            hstatus: Hstatus {},
            hie: Hie {},
            hvip: Hvip {},
            hgatp: Hgatp {},
            hgeie: Hgeie {},
            htval: Htval {},
            hideleg: Hideleg {},
            hedeleg: Hedeleg {},
            hcounteren: Hcounteren {},
            vsatp: Vsatp {},
            vstval: Vstval {},
        }
    }

    /* hypervisorモードの指定割込みを有効化 */
    pub fn enable_mask_h(&self, int_mask: usize) {
        let hint_mask = 0x1444 & int_mask; // hieの有効ビットでマスク
        self.hie.set(self.hie.get() | hint_mask as u64);
    }

    /* hypervisorモードの指定割込みを無効化 */
    pub fn disable_mask_h(&self, int_mask: usize) {
        let hint_mask = 0x1444 & int_mask; // hieの有効ビットでマスク
        self.hie.set(self.hie.get() & !(hint_mask as u64));
    }

    /* VS-modeへの割込み移譲を設定 */
    pub fn set_delegation_int(&self, int_mask: usize) {
        self.hideleg.set(self.hideleg.get() | int_mask as u64);
    }

    /* VS-modeへの割込み移譲を解除 */
    pub fn clear_delegation_int(&self, int_mask: usize) {
        self.hideleg.set(self.hideleg.get() & !(int_mask as u64));
    }

    /* VS-modeへの例外移譲を設定 */
    pub fn set_delegation_exc(&self, exc_mask: usize) {
        self.hedeleg.set(self.hedeleg.get() | exc_mask as u64);
    }

    /* VS-modeへの例外移譲を解除 */
    pub fn clear_delegation_exc(&self, exc_mask: usize) {
        self.hedeleg.set(self.hedeleg.get() & !(exc_mask as u64));
    }

    /* VS-modeに仮想割込みを発生させる */
    pub fn assert_vsmode_interrupt(&self, int_mask: usize) {
        self.hvip.set(int_mask as u64);
    }

    /* VS-modeの割込みをクリアする */
    pub fn flush_vsmode_interrupt(&self, int_mask: usize) {
        let mask = !(int_mask) & self.hvip.get() as usize;
        self.hvip.set(mask as u64);
    }

    /* 指定外部割込みの有効化  */
    pub fn enable_exint_mask_h(&self, int_mask: usize) {
        self.hgeie.set(self.hgeie.get() | int_mask as u64);
    }

    /* 指定外部割込みの無効化 */
    pub fn disable_exint_mask_h(&self, int_mask: usize) {
        self.hgeie.set(self.hgeie.get() & !(int_mask as u64));
    }

    /* VS-modeのcounterenレジスタを設定 */
    pub fn enable_vsmode_counter_access(&self, counter_mask: usize) {
        self.hcounteren
            .set(self.hcounteren.get() | counter_mask as u32);
    }

    /* VS-modeのcounterenレジスタをクリア */
    pub fn disable_vsmode_counter_access(&self, counter_mask: usize) {
        self.hcounteren
            .set(self.hcounteren.get() & !(counter_mask as u32));
    }

    /* HS-modeが用意するページテーブルのモードを設定 */
    pub fn set_paging_mode_hv(&self, mode: PagingMode) {
        match mode {
            PagingMode::Bare => {
                self.hgatp.modify(hgatp::MODE::BARE);
            }
            PagingMode::Sv39x4 => {
                self.hgatp.modify(hgatp::MODE::SV39X4);
            }
            PagingMode::Sv48x4 => {
                self.hgatp.modify(hgatp::MODE::SV48X4);
            }
            PagingMode::Sv57x4 => {
                self.hgatp.modify(hgatp::MODE::SV57X4);
            }
        };
    }

    /* HS-modeが用意するページテーブルのアドレスを設定 */
    pub fn set_table_addr_hv(&self, table_addr: usize) {
        self.hgatp.modify(hgatp::PPN::CLEAR);
        let current = self.hgatp.get();
        self.hgatp
            .set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    /* ページテーブルのアドレスを取得する */
    pub fn get_hs_pagetable(&self) -> u64 {
        (self.hgatp.get() & 0x0fff_ffff_ffff) << 12
    }

    /* ページテーブルのアドレスを取得する */
    pub fn set_vs_pagetable(&self, table_addr: usize) {
        self.vsatp.modify(vsatp::PPN::CLEAR);
        let current = self.vsatp.get();
        self.vsatp
            .set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    /* ページテーブルのアドレスを取得する */
    pub fn get_vs_pagetable(&self) -> u64 {
        (self.vsatp.get() & 0x0fff_ffff_ffff) << 12
    }

    /* ページフォルト時のアドレスを取得する */
    pub fn get_vs_fault_address(&self) -> u64 {
        self.vstval.get()
    }

    /* ページテーブルのアドレスを取得する */
    pub fn get_vs_fault_paddr(&self) -> u64 {
        self.htval.get() << 2
    }
}
