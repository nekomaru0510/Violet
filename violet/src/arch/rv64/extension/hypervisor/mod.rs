//! Hypervisor Extension

use crate::arch::rv64;
use crate::arch::traits::hypervisor::HypervisorT;
use crate::arch::traits::TraitArch;
use rv64::Rv64;
use rv64::trap::exc::Exception;
use rv64::trap::int::Interrupt;
use rv64::trap::TrapVector;
use rv64::vscontext::VsContext;
use rv64::PagingMode;

use rv64::csr::hcounteren::*;
use rv64::csr::hedeleg::*;
use rv64::csr::hgatp;
use rv64::csr::hgatp::*;
use rv64::csr::hgeie::*;
use rv64::csr::hideleg::*;
use rv64::csr::hie::*;
use rv64::csr::htval::*;
use rv64::csr::hvip::*;
use rv64::csr::vsatp::*;
use rv64::csr::vsatp;
use rv64::csr::vstval::*;
use rv64::csr::vstvec::*;

#[derive(Clone)]
pub struct Hext {}

impl HypervisorT for Hext {
    type Context = VsContext;
    fn init() {
        Hext::init();
    }

    fn hook(vecid: usize, func: fn(regs: *mut usize)) {
        if vecid > TrapVector::INTERRUPT_OFFSET {
            Self::clear_delegation_int(Interrupt::bit(vecid));
        } else {
            Self::clear_delegation_exc(Exception::bit(vecid));
        }
        let _ = Rv64::register_vector(vecid, func);
    }

    fn mmu_enable() {
        enable_paging();
    }
}

impl Hext {
    pub fn init() {
        Self::set_delegation_exc(
            Exception::bit(Exception::INSTRUCTION_ADDRESS_MISALIGNED)
                | Exception::bit(Exception::BREAKPOINT)
                | Exception::bit(Exception::ENVIRONMENT_CALL_FROM_UMODE_OR_VUMODE)
                | Exception::bit(Exception::INSTRUCTION_PAGE_FAULT)
                | Exception::bit(Exception::LOAD_PAGE_FAULT)
                | Exception::bit(Exception::STORE_AMO_PAGE_FAULT),
        );

        Self::set_delegation_int(
            Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_SOFTWARE_INTERRUPT)
                | Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT)
                | Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT),
        );

        Self::flush_vsmode_interrupt(0xffff_ffff_ffff_ffff);
        Self::enable_vsmode_counter_access(0xffff_ffff);
    }

    /* hypervisorモードの指定割込みを有効化 */
    pub fn enable_mask_h(int_mask: usize) {
        let hint_mask = 0x1444 & int_mask; // hieの有効ビットでマスク
        Hie::set(Hie::get() | hint_mask as u64);
    }

    /* hypervisorモードの指定割込みを無効化 */
    pub fn disable_mask_h(int_mask: usize) {
        let hint_mask = 0x1444 & int_mask; // hieの有効ビットでマスク
        Hie::set(Hie::get() & !(hint_mask as u64));
    }

    /* VS-modeへの割込み移譲を設定 */
    pub fn set_delegation_int(int_mask: usize) {
        Hideleg::set(Hideleg::get() | int_mask as u64);
    }

    /* VS-modeへの割込み移譲を解除 */
    pub fn clear_delegation_int(int_mask: usize) {
        Hideleg::set(Hideleg::get() & !(int_mask as u64));
    }

    /* VS-modeへの例外移譲を設定 */
    pub fn set_delegation_exc(exc_mask: usize) {
        Hedeleg::set(Hedeleg::get() | exc_mask as u64);
    }

    /* VS-modeへの例外移譲を解除 */
    pub fn clear_delegation_exc(exc_mask: usize) {
        Hedeleg::set(Hedeleg::get() & !(exc_mask as u64));
    }

    /* VS-modeに仮想割込みを発生させる */
    pub fn assert_vsmode_interrupt(int_mask: usize) {
        Hvip::set(int_mask as u64);
    }

    /* VS-modeの割込みをクリアする */
    pub fn flush_vsmode_interrupt(int_mask: usize) {
        let mask = !(int_mask) & Hvip::get() as usize;
        Hvip::set(mask as u64);
    }

    /* 指定外部割込みの有効化  */
    pub fn enable_exint_mask_h(int_mask: usize) {
        Hgeie::set(Hgeie::get() | int_mask as u64);
    }

    /* 指定外部割込みの無効化 */
    pub fn disable_exint_mask_h(int_mask: usize) {
        Hgeie::set(Hgeie::get() & !(int_mask as u64));
    }

    /* VS-modeのcounterenレジスタを設定 */
    pub fn enable_vsmode_counter_access(counter_mask: usize) {
        Hcounteren::set(Hcounteren::get() | counter_mask as u32);
    }

    /* VS-modeのcounterenレジスタをクリア */
    pub fn disable_vsmode_counter_access(counter_mask: usize) {
        Hcounteren::set(Hcounteren::get() & !(counter_mask as u32));
    }

    /* HS-modeが用意するページテーブルのモードを設定 */
    pub fn set_paging_mode_hv(mode: PagingMode) {
        match mode {
            PagingMode::Bare => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::BARE);
            }
            PagingMode::Sv39x4 => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::SV39X4);
            }
            PagingMode::Sv48x4 => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::SV48X4);
            }
            PagingMode::Sv57x4 => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::SV57X4);
            }
        };
    }

    /* HS-modeが用意するページテーブルのアドレスを設定 */
    pub fn set_table_addr_hv(table_addr: usize) {
        Hgatp::write(hgatp::PPN, hgatp::PPN::CLEAR);
        let current = Hgatp::get();
        Hgatp::set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    /* ページテーブルのアドレスを取得する */
    pub fn get_hs_pagetable() -> u64 {
        (Hgatp::get() & 0x0fff_ffff_ffff) << 12
    }

    /* ページテーブルのアドレスを取得する */
    pub fn set_vs_pagetable(table_addr: usize) {
        Vsatp::write(vsatp::PPN, vsatp::PPN::CLEAR);
        let current = Vsatp::get();
        Vsatp::set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    /* ページテーブルのアドレスを取得する */
    pub fn get_vs_pagetable() -> u64 {
        (Vsatp::get() & 0x0fff_ffff_ffff) << 12
    }

    /* ページフォルト時のアドレスを取得する */
    pub fn get_vs_fault_address() -> u64 {
        Vstval::get()
    }

    /* ページテーブルのアドレスを取得する */
    pub fn get_vs_fault_paddr() -> u64 {
        Htval::get() << 2
    }

    /* VS-modeのstvecを取得 */
    pub fn set_vs_vector(val: u64) {
        Vstvec::set(val);
    }

    /* VS-modeのstvecを取得 */
    pub fn get_vs_vector() -> u64 {
        Vstvec::get()
    }
}

extern crate core;
use core::intrinsics::transmute;

use crate::arch::rv64::mmu::sv48::PageTableSv48;
use crate::arch::traits::mmu::{PageEntry, PageTable};

static mut PAGE_TABLE_ARRAY: [PageTableSv48; MAX_PAGE_TABLE] =
    [PageTableSv48::empty(); MAX_PAGE_TABLE];
const MAX_PAGE_TABLE: usize = 32; //16;
static mut PAGE_TABLE_IDX: usize = 0;

pub fn enable_paging() {
    Hext::set_vs_pagetable(0);
    Hext::set_table_addr_hv(unsafe { transmute(&PAGE_TABLE_ARRAY[0]) });
    Hext::set_paging_mode_hv(PagingMode::Sv48x4);
}

pub fn create_page_table() {
    for i in 0..0x100 {
        unsafe {
            map_vaddr::<PageTableSv48>(
                &mut PAGE_TABLE_ARRAY[0],
                0x8020_0000 + i * 0x1000,
                0x8020_0000 + i * 0x1000,
            );
        }
    }
}

/* 仮想アドレス->物理アドレスへの変更 */
pub fn to_paddr<T: PageTable>(table: &mut T, vaddr: usize) -> usize {
    match (*table).get_page_entry(vaddr) {
        None => 0,
        Some(e) => ((e.get_ppn() << 12) as usize) | (vaddr & 0x0fff),
    }
}

/* 指定仮想アドレス領域の無効化 */
pub fn invalid_page<T: PageTable>(table: &mut T, vaddr: usize) {
    table.get_page_entry(vaddr).unwrap().invalid();
}

pub fn map_vaddr<T: PageTable>(table: &mut T, paddr: usize, vaddr: usize) {
    for idx in 1..5 {
        match (*table).create_page_entry(paddr, vaddr) {
            Ok(()) => break,
            Err(i) => unsafe {
                match (*table).get_table(vaddr, i) {
                    None => return,
                    Some(t) => {
                        t.get_entry(vaddr, i)
                            .set_paddr(transmute(&mut PAGE_TABLE_ARRAY[PAGE_TABLE_IDX + 1]));
                        t.get_entry(vaddr, i).valid();
                        PAGE_TABLE_IDX = PAGE_TABLE_IDX + 1;
                        if MAX_PAGE_TABLE < PAGE_TABLE_IDX {
                            loop {}
                        }
                    }
                }
            },
        }
    }
}
