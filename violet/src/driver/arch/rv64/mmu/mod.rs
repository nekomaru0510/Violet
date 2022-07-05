//! メモリ管理
extern crate register;
use register::cpu::RegisterReadWrite;

use crate::driver::traits::arch::riscv::PagingMode;

use super::csr::satp::*;

pub mod sv39;
pub mod sv48;

#[derive(Clone)]
pub struct Rv64Mmu {
    pub satp: Satp,
}

impl Rv64Mmu {
    pub const fn new() -> Self {
        Rv64Mmu { satp: Satp {} }
    }

    //MMU 関連
    // ページングモードの設定
    pub fn set_paging_mode(&self, mode: PagingMode) {
        match mode {
            PagingMode::Bare => {
                self.satp.modify(satp::MODE::BARE);
            }
            PagingMode::Sv39x4 => {
                self.satp.modify(satp::MODE::SV39X4);
            }
            PagingMode::Sv48x4 => {
                self.satp.modify(satp::MODE::SV48X4);
            }
            PagingMode::Sv57x4 => {
                self.satp.modify(satp::MODE::SV57X4);
            }
        };
    }

    // ページテーブルのアドレスを設定
    pub fn set_table_addr(&self, table_addr: usize) {
        self.satp.modify(satp::PPN::CLEAR);
        let current = self.satp.get();
        self.satp
            .set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
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
