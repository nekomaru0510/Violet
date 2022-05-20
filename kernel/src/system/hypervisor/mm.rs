//! メモリ管理

extern crate core;
use core::intrinsics::transmute;

use crate::environment::traits::cpu::HasCpu;
use crate::PERIPHERALS;

//use crate::driver::traits::arch::riscv::{PageEntry, PageTable, PagingMode};
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

/*
fn _map_vaddr(paddr: u64, vaddr: u64) {
    unsafe {
        map_vaddr(&mut PAGE_TABLE_ARRAY[0], paddr, vaddr);
    }
}
*/
// sizeは4KiB固定
pub fn map_vaddr(table_top: &mut PageTableSv39, paddr: u64, vaddr: u64) {
    unsafe {
        let mut vpn;
        let mut ppn;
        let mut table: &mut PageTableSv39 = table_top; // satpからとってくる？
        let mut entry: &mut PageEntrySv39;

        for i in (0..3).rev() {
            // vpnの更新
            vpn = SV39_VA.vpn[i].mask(vaddr);
            // ppnの更新
            ppn = SV39_PA.ppn[i].mask(paddr);
            // 次のエントリを取得
            //entry = *table.get_entry(vpn);
            entry = &mut ((*table).entry[vpn as usize]);

            if (i == 0) {
                //entry.set_ppn(paddr >> 12);
                entry.set_parmition(1);
            } else {
                // 次のテーブルを取得
                table = transmute((*table).get_entry_ppn(vpn) << 12);
            }
        }
    }
}

pub fn map_vaddr48(table_top: &mut PageTableSv48, paddr: u64, vaddr: u64) {
    unsafe {
        let mut vpn;
        let mut ppn;
        let mut table: &mut PageTableSv48 = table_top; // satpからとってくる？
        let mut entry: &mut PageEntrySv48;

        for i in (0..4).rev() {
            // vpnの更新
            vpn = SV48_VA.vpn[i].mask(vaddr);
            // ppnの更新
            ppn = SV48_PA.ppn[i].mask(paddr);
            // 次のエントリを取得
            //entry = *table.get_entry(vpn);
            entry = &mut ((*table).entry[vpn as usize]);

            if (i == 0) {
                //entry.set_ppn(paddr >> 12);
                entry.set_parmition(1);
            } else {
                // 次のテーブルを取得
                table = transmute((*table).get_entry_ppn(vpn) << 12);
            }
        }
    }
}

pub fn __map_vaddr(table_top: &mut PageTableSv39, paddr: u64, vaddr: u64) {
    unsafe {
        let mut vpn;
        let mut ppn;
        let mut table: &mut PageTableSv39 = table_top; // satpからとってくる？
        let mut entry: &mut PageEntrySv39;

        for i in (0..3).rev() {
            // vpnの更新
            vpn = SV39_VA.vpn[i].mask(vaddr);
            // ppnの更新
            ppn = SV39_PA.ppn[i].mask(paddr);
            // 次のエントリを取得
            //entry = *table.get_entry(vpn);
            entry = &mut ((*table).entry[vpn as usize]);

            // 当該エントリが有効でない場合
            if (!(entry.is_valid())) {
                // エントリの設定
                entry.valid();

                // 次のテーブルを作成する
                if (i != 0) {
                    // 次のテーブルの作成
                    PAGE_TABLE_IDX = PAGE_TABLE_IDX + 1; //この方法でのテーブル作成は危険なので、やめる
                                                         // 次のテーブルを設定
                    let mut addr: u64 = transmute(&PAGE_TABLE_ARRAY[PAGE_TABLE_IDX]);
                    entry.set_ppn(addr >> 12);
                }
                // 次のテーブルが存在しない
                else {
                    entry.writable();
                    entry.set_ppn(paddr >> 12);
                    //break;
                }
            }

            // 次のテーブルを取得
            table = transmute((*table).get_entry_ppn(vpn) << 12);
        }
    }
}
