//! Module that handles MMU related processing

use crate::arch::rv64::PagingMode;
use super::csr::satp::*;

pub mod sv39;
pub mod sv48;

use core::intrinsics::transmute;
use crate::arch::rv64::mmu::sv48::PageTableSv48;
use crate::arch::traits::mmu::TraitMmu;

type PageTable = PageTableSv48;
static mut PAGE_TABLE_ARRAY: [PageTable; MAX_PAGE_TABLE] =
    [PageTable::empty(); MAX_PAGE_TABLE];
pub const MAX_PAGE_TABLE: usize = 32; //16;
static mut PAGE_TABLE_IDX: usize = 0;

pub fn get_new_page_table_idx() -> usize {    
    unsafe {
        PAGE_TABLE_IDX = PAGE_TABLE_IDX + 1;
        if MAX_PAGE_TABLE < PAGE_TABLE_IDX {
            panic!("get_new_page_table_idx: out of range");
        }
        PAGE_TABLE_IDX
    }    
}

pub fn get_page_table_addr(idx: usize) -> usize {
    if MAX_PAGE_TABLE < idx {
        return 0;
    }
    unsafe { transmute(&PAGE_TABLE_ARRAY[idx]) }
}

pub fn get_new_page_table_addr() -> usize {
    let idx = get_new_page_table_idx();
    if MAX_PAGE_TABLE < idx {
        panic!("get_new_page_table_addr: out of range");
    }
    unsafe { transmute(&PAGE_TABLE_ARRAY[idx]) }
}

pub fn get_page_table(idx: usize) -> &'static PageTable {
    if MAX_PAGE_TABLE < idx {
        panic!("get_page_table: out of range");
    }
    unsafe { transmute(&PAGE_TABLE_ARRAY[idx]) }
}

pub fn get_mut_page_table(idx: usize) -> &'static mut PageTable {
    if MAX_PAGE_TABLE < idx {
        panic!("get_page_table: out of range");
    }
    unsafe { transmute(&mut PAGE_TABLE_ARRAY[idx]) }
}

enum Rv64PageTable {
    Sv39(sv39::PageTableSv39),
    Sv48(sv48::PageTableSv48),
}

#[derive(Clone)]
pub struct Rv64Mmu {}

impl TraitMmu for Rv64Mmu {
    fn enable_mmu() {
        Rv64Mmu::set_paging_mode(PagingMode::Sv48x4);
    }
}

impl Rv64Mmu {
    pub const fn new() -> Self {
        Rv64Mmu {}
    }

    //MMU 関連
    // ページングモードの設定
    pub fn set_paging_mode(mode: PagingMode) {
        match mode {
            PagingMode::Bare => {
                Satp::write(MODE, MODE::BARE);
            }
            PagingMode::Sv39x4 => {
                Satp::write(MODE, MODE::SV39X4);
            }
            PagingMode::Sv48x4 => {
                Satp::write(MODE, MODE::SV48X4);
            }
            PagingMode::Sv57x4 => {
                Satp::write(MODE, MODE::SV57X4);
            }
        };
    }

    // ページングモードの取得
    pub fn get_paging_mode() -> PagingMode {
        match Satp::read(MODE) {
            MODE::BARE => PagingMode::Bare,
            MODE::SV39X4 => PagingMode::Sv39x4,
            MODE::SV48X4 => PagingMode::Sv48x4,
            MODE::SV57X4 => PagingMode::Sv57x4,
            _ => PagingMode::Bare,
        }
    }

    // ページテーブルのアドレスを設定
    pub fn set_table_addr(&self, table_addr: usize) {
        Satp::write(PPN, PPN::CLEAR);
        let current = Satp::get();
        Satp::set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    // ページテーブルのアドレスを取得
    pub fn get_table_addr() -> usize {
        (Satp::read(PPN) << 12) as usize
    }
}

// ビットフィールド操作用[todo fix 場所の変更]
pub struct BitField {
    pub offset: u8,
    pub width: u8,
}
impl BitField {
    pub fn new() -> Self {
        BitField {
            offset: 0,
            width: 0,
        }
    }

    /* offsetとwidthに沿ったビットパターンを生成 */
    /* valはT型にしたい */
    pub fn pattern(&self, val: u64) -> u64 {
        let mask = (2 << (self.width - 1)) - 1;
        (val & mask) << self.offset
    }

    /* offsetとwidthに沿ったビットパターンを抽出 */
    /* valはT型にしたい */
    pub fn mask(&self, val: u64) -> u64 {
        let mask = (2 << (self.width - 1)) - 1;
        (val >> self.offset) & mask
    }
}
