//! Module that handles MMU related processing

use crate::arch::rv64::PagingMode;
use super::csr::satp::*;

pub mod sv39;
pub mod sv48;

use core::intrinsics::transmute;
use crate::arch::rv64::mmu::sv48::PageTableSv48;
use crate::arch::traits::mmu::TraitMmu;

type PageTable = PageTableSv48;
use core::sync::atomic::{AtomicUsize, Ordering};

pub const MAX_PAGE_TABLE: usize = 32;
static mut PAGE_TABLE_ARRAY: [PageTable; MAX_PAGE_TABLE] =
    [PageTable::empty(); MAX_PAGE_TABLE];
static PAGE_TABLE_IDX: AtomicUsize = AtomicUsize::new(0);

#[repr(C, align(16384))]
#[derive(Clone, Copy)]
struct RootPageTable {
    tables: [PageTable; 4],
}
static mut ROOT_PAGE_TABLES: [RootPageTable; crate::environment::NUM_OF_CPUS] =
    [RootPageTable { tables: [PageTable::empty(); 4] }; crate::environment::NUM_OF_CPUS];
static ROOT_PAGE_TABLE_IDX: AtomicUsize = AtomicUsize::new(0);

fn reserve(next: &AtomicUsize, limit: usize) -> usize {
    next.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |index| {
        if index < limit { Some(index + 1) } else { None }
    }).expect("Page table pool exhausted")
}
pub fn get_new_page_table_idx() -> usize {
    reserve(&PAGE_TABLE_IDX, MAX_PAGE_TABLE)
}
pub fn get_page_table_addr(idx: usize) -> usize {
    assert!(idx < MAX_PAGE_TABLE, "Page table index out of range");
    unsafe { core::ptr::addr_of!(PAGE_TABLE_ARRAY[idx]) as usize }
}
pub fn get_new_page_table_addr() -> usize {
    get_page_table_addr(get_new_page_table_idx())
}
pub fn get_new_root_page_table_addr_x4() -> usize {
    let index = reserve(&ROOT_PAGE_TABLE_IDX, crate::environment::NUM_OF_CPUS);
    unsafe { core::ptr::addr_of!(ROOT_PAGE_TABLES[index]) as usize }
}
pub fn get_page_table(idx: usize) -> &'static PageTable {
    unsafe { &*(get_page_table_addr(idx) as *const PageTable) }
}
pub fn get_mut_page_table(idx: usize) -> &'static mut PageTable {
    unsafe { &mut *(get_page_table_addr(idx) as *mut PageTable) }
}

#[test_case]
fn page_table_layout() -> Result<(), &'static str> {
    assert_eq!(core::mem::size_of::<PageTable>(), 4096);
    assert_eq!(core::mem::align_of::<PageTable>(), 4096);
    assert_eq!(core::mem::size_of::<RootPageTable>(), 16384);
    assert_eq!(core::mem::align_of::<RootPageTable>(), 16384);
    Ok(())
}
#[test_case]
fn page_table_slot_bounds() -> Result<(), &'static str> {
    let next = AtomicUsize::new(0);
    assert_eq!(reserve(&next, 2), 0);
    assert_eq!(reserve(&next, 2), 1);
    assert_eq!(next.load(Ordering::Relaxed), 2);
    Ok(())
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

    pub fn get_paging_mode() -> PagingMode {
        match Satp::read(MODE) {
            MODE::BARE => PagingMode::Bare,
            MODE::SV39X4 => PagingMode::Sv39x4,
            MODE::SV48X4 => PagingMode::Sv48x4,
            MODE::SV57X4 => PagingMode::Sv57x4,
            _ => PagingMode::Bare,
        }
    }

    pub fn set_table_addr(&self, table_addr: usize) {
        Satp::write(PPN, PPN::CLEAR);
        let current = Satp::get();
        Satp::set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    pub fn get_table_addr() -> usize {
        (Satp::read(PPN) << 12) as usize
    }
}

