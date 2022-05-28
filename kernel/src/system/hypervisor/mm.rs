//! メモリ管理

extern crate core;
use core::intrinsics::transmute;

use crate::environment::traits::cpu::HasCpu;
use crate::PERIPHERALS;

//use crate::driver::traits::arch::riscv::{PageEntry, PageTable, PagingMode};
use crate::driver::traits::cpu::mmu::{PageEntry, PageTable, TraitMmu};
use crate::driver::arch::rv64::mm::sv39::{PageEntrySv39, PageTableSv39, SV39_PA, SV39_VA}; /* todo delete*/
use crate::driver::arch::rv64::mm::sv48::{PageEntrySv48, PageTableSv48, SV48_PA, SV48_VA};
use crate::driver::traits::arch::riscv::*; /* todo delete*/

const MAX_PAGE_TABLE: usize = 16;
//static mut PAGE_TABLE_ARRAY: [PageTableSv39; MAX_PAGE_TABLE] = [PageTableSv39::empty(); MAX_PAGE_TABLE];
static mut PAGE_TABLE_ARRAY: [PageTableSv48; MAX_PAGE_TABLE] =
    [PageTableSv48::empty(); MAX_PAGE_TABLE];
static mut PAGE_TABLE_IDX: usize = 0;

pub fn enable_paging() {
    let cpu = unsafe { PERIPHERALS.take_cpu() };

    unsafe {
        cpu.set_table_addr(transmute(&PAGE_TABLE_ARRAY[0]));
    }
    cpu.set_paging_mode(PagingMode::Sv39x4);

    unsafe { PERIPHERALS.release_cpu(cpu) };
}

pub fn create_page_table() {
    //map_vaddr(0x8200_0000, 0xC000_0000);
    /*
    for i in (0..0x10_0000) {
        map_vaddr(i*0x1000, i*0x1000);
        //map_vaddr(0x8010_0000 + i*0x1000, 0x8010_0000 + i*0x1000);
    }
     */

    //_map_vaddr(0x8010_0000, 0x8010_0000);

    /*
    map_vaddr(0x8010_0000, 0x8010_0000);
    map_vaddr(0x8010_1000, 0x8010_1000);
    map_vaddr(0x8010_2000, 0x8010_2000);
    map_vaddr(0x8010_3000, 0x8010_3000);
    map_vaddr(0x8010_4000, 0x8010_4000);
    */
}

pub fn _map_vaddr48<T: PageTable>(table: &mut T, vaddr: usize) {
    table.get_page_entry(vaddr).invalid();
}
