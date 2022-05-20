//! Sv39用ページエントリ・ページテーブル

use super::BitField;
use crate::driver::traits::arch::riscv::{PageEntry, PageTable};

// 仮想アドレスのビットフィールド
pub struct VirtualAddressFieldSv39 {
    pub page_offset: BitField,
    pub vpn: [BitField; 3],
}

// 物理アドレスのビットフィールド
pub struct PhysicalAddressFieldSv39 {
    pub page_offset: BitField,
    pub ppn: [BitField; 3],
}

//ページエントリのビットフィールド
pub struct PageEntryFieldSv39 {
    pub v: BitField,
    pub r: BitField,
    pub w: BitField,
    pub x: BitField,
    pub u: BitField,   // U-mode Access
    pub g: BitField,   // Global Mapping
    pub a: BitField,   //
    pub d: BitField,   // Dirty Bit
    pub rsw: BitField, // Reserved
    //pub ppn : [BitField; 3],
    pub ppn: BitField, //
}

pub const SV39_VA: VirtualAddressFieldSv39 = VirtualAddressFieldSv39 {
    page_offset: BitField {
        offset: 0,
        width: 12,
    },
    vpn: [
        BitField {
            offset: 12,
            width: 9,
        },
        BitField {
            offset: 21,
            width: 9,
        },
        BitField {
            offset: 30,
            width: 9,
        },
    ],
};
pub const SV39_PA: PhysicalAddressFieldSv39 = PhysicalAddressFieldSv39 {
    page_offset: BitField {
        offset: 0,
        width: 12,
    },
    ppn: [
        BitField {
            offset: 12,
            width: 9,
        },
        BitField {
            offset: 21,
            width: 9,
        },
        BitField {
            offset: 30,
            width: 26,
        },
    ],
};
pub const SV39_ENTRY: PageEntryFieldSv39 = PageEntryFieldSv39 {
    v: BitField {
        offset: 0,
        width: 1,
    },
    r: BitField {
        offset: 1,
        width: 1,
    },
    w: BitField {
        offset: 2,
        width: 1,
    },
    x: BitField {
        offset: 3,
        width: 1,
    },
    u: BitField {
        offset: 4,
        width: 1,
    }, // U-mode Access
    g: BitField {
        offset: 5,
        width: 1,
    }, // Global Mapping
    a: BitField {
        offset: 6,
        width: 1,
    }, //
    d: BitField {
        offset: 7,
        width: 1,
    }, // Dirty Bit
    rsw: BitField {
        offset: 8,
        width: 2,
    }, // Reserved
    /*
    ppn : [
        BitField{offset:10, width: 9},
        BitField{offset:29, width: 9},
        BitField{offset:28, width: 26},
        ],
    */
    ppn: BitField {
        offset: 10,
        width: 44,
    },
};

// ページエントリ(Sv39)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageEntrySv39 {
    entry: u64,
}

impl PageEntrySv39 {
    pub const fn empty() -> PageEntrySv39 {
        PageEntrySv39 { entry: 0 }
    }
}

impl PageEntry for PageEntrySv39 {
    fn new() -> Self {
        PageEntrySv39 { entry: 0 }
    }

    // todo fix
    fn set_parmition(&mut self, flags: usize) {
        self.entry &= !SV39_ENTRY.r.pattern(1);
    }

    /* 当該ページ物理アドレスor次のテーブルのアドレスを設定する */
    fn set_ppn(&mut self, ppn: u64) {
        self.entry &= !SV39_ENTRY.ppn.pattern(0xffff_ffff_ffff_ffff); // PPN0クリア
        self.entry |= SV39_ENTRY.ppn.pattern(ppn);
    }

    /* 当該ページ物理アドレスor次のテーブルのアドレスを設定する */
    fn get_ppn(&self) -> u64 {
        SV39_ENTRY.ppn.mask(self.entry)
    }

    fn is_valid(&mut self) -> bool {
        SV39_ENTRY.v.mask(self.entry) == 1
    }

    /* 当該ページの有効化 */
    fn valid(&mut self) {
        self.entry |= SV39_ENTRY.v.pattern(1);
    }

    /* 当該ページの無効化 */
    fn invalid(&mut self) {
        self.entry &= !SV39_ENTRY.v.pattern(0xffff_ffff_ffff_ffff); // PPN0クリア
    }

    fn writable(&mut self) {
        self.entry |= SV39_ENTRY.w.pattern(1);
        self.entry |= SV39_ENTRY.r.pattern(1);
        self.entry |= SV39_ENTRY.x.pattern(1);
        //self.entry |= SV39_ENTRY.g.pattern(1);
    }
}

#[repr(C)]
#[derive(Clone, Copy)] //危険か？
#[repr(align(4096))]
pub struct PageTableSv39 {
    pub entry: [PageEntrySv39; 512],
}

impl PageTableSv39 {
    pub const fn empty() -> Self {
        PageTableSv39 {
            entry: [PageEntrySv39::empty(); 512], //entry: [0; 512]
        }
    }
}

impl PageTable for PageTableSv39 {
    type Entry = PageEntrySv39;

    // 初期化
    fn new() -> Self {
        PageTableSv39 {
            entry: [PageEntrySv39::new(); 512],
        }
    }

    // ページエントリを取得
    fn get_entry(&mut self, vpn: u64) -> &mut <Self as PageTable>::Entry {
        &mut self.entry[vpn as usize]
    }

    // ページエントリのアドレスを取得
    fn get_entry_ppn(&self, vpn: u64) -> u64 {
        let e = self.entry[vpn as usize];
        e.get_ppn()
    }
}
