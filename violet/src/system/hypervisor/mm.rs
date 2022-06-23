//! メモリ管理

extern crate core;
use core::intrinsics::transmute;

use crate::CPU;

use crate::driver::traits::cpu::mmu::{PageEntry, PageTable};
use crate::driver::arch::rv64::mmu::sv48::{PageTableSv48};
use crate::driver::traits::arch::riscv::*; /* todo delete*/

use crate::{print,println};

const MAX_PAGE_TABLE: usize = 16;
static mut PAGE_TABLE_ARRAY: [PageTableSv48; MAX_PAGE_TABLE] =
    [PageTableSv48::empty(); MAX_PAGE_TABLE];
static mut PAGE_TABLE_IDX: usize = 0;

pub fn enable_paging() {
    CPU.mmu.set_table_addr( unsafe {transmute(&PAGE_TABLE_ARRAY[0])} );
    CPU.mmu.set_paging_mode(PagingMode::Sv48x4);
}

pub fn create_page_table() {
    for i in (0..0x10) {
        unsafe {
            map_vaddr::<PageTableSv48>(&mut PAGE_TABLE_ARRAY[0], 0x8010_0000 + i*0x1000, 0x8010_0000 + i*0x1000);
        }
    }
}

/* 仮想アドレス->物理アドレスへの変更 */
pub fn to_paddr<T: PageTable>(table: &mut T, vaddr: usize) -> usize {
    match (*table).get_page_entry(vaddr) {
        None => 0,
        Some(e) => ((e.get_ppn() << 12) as usize ) | (vaddr & 0x0fff)
    }
}

/* 指定仮想アドレス領域の無効化 */
pub fn invalid_page<T: PageTable>(table: &mut T, vaddr: usize) {    
    table.get_page_entry(vaddr).unwrap().invalid();
}

pub fn map_vaddr<T: PageTable>(table: &mut T, paddr: usize, vaddr: usize) {
    for idx in (1..5) {
        match (*table).create_page_entry(paddr, vaddr) {
            Ok(()) => {
                break
            },
            Err(i) => {
                unsafe {
                    match (*table).get_table(vaddr, i) {
                        None => return,
                        Some(t) => {
                            t.get_entry(vaddr, i).set_paddr(transmute(&mut PAGE_TABLE_ARRAY[PAGE_TABLE_IDX+1]) );
                            t.get_entry(vaddr, i).valid();
                            PAGE_TABLE_IDX = PAGE_TABLE_IDX + 1;
                        }
                    }
                }
            }
        }
    }
}

