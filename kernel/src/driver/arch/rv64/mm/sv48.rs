//! Sv48用ページエントリ・ページテーブル

use super::BitField;
use crate::driver::traits::arch::riscv::{PageEntry, PageTable};

// 仮想アドレスのビットフィールド
pub struct VirtualAddressFieldSv48 {
    pub page_offset : BitField,
    pub vpn : [BitField; 4],
}

// 物理アドレスのビットフィールド
pub struct PhysicalAddressFieldSv48 {
    pub page_offset : BitField,
    pub ppn : [BitField; 4],
}

//ページエントリのビットフィールド
pub struct PageEntryFieldSv48 {
    pub v : BitField,
    pub r : BitField,
    pub w : BitField,
    pub x : BitField,
    pub u : BitField, // U-mode Access
    pub g : BitField, // Global Mapping
    pub a : BitField, // 
    pub d : BitField, // Dirty Bit
    pub rsw : BitField, // Reserved
    //pub ppn : [BitField; 3],
    pub ppn : BitField, // 
    pub pbmt : BitField, // 
    pub n : BitField, // 
}


pub const SV48_VA: VirtualAddressFieldSv48 = VirtualAddressFieldSv48 {
    page_offset : BitField{offset:0, width: 12},
    vpn : [ 
        BitField{offset:12, width: 9}, 
        BitField{offset:21, width: 9}, 
        BitField{offset:30, width: 9},
        BitField{offset:39, width: 9},
        ],
};
pub const SV48_PA: PhysicalAddressFieldSv48 = PhysicalAddressFieldSv48 {
    page_offset : BitField{offset:0, width: 12},
    ppn : [ 
        BitField{offset:12, width: 9}, 
        BitField{offset:21, width: 9}, 
        BitField{offset:30, width: 9},
        BitField{offset:39, width:17},
        ],
};
pub const SV48_ENTRY: PageEntryFieldSv48 = PageEntryFieldSv48 {
    v : BitField{offset:0, width: 1},
    r : BitField{offset:1, width: 1},
    w : BitField{offset:2, width: 1},
    x : BitField{offset:3, width: 1},
    u : BitField{offset:4, width: 1}, // U-mode Access
    g : BitField{offset:5, width: 1}, // Global Mapping
    a : BitField{offset:6, width: 1}, // 
    d : BitField{offset:7, width: 1}, // Dirty Bit
    rsw : BitField{offset:8, width: 2}, // Reserved
    /*
    ppn : [ 
        BitField{offset:10, width: 9}, 
        BitField{offset:29, width: 9}, 
        BitField{offset:28, width: 26},
        ],
    */
    ppn : BitField{offset:10, width: 44},
    pbmt: BitField{offset:61, width: 2},
    n : BitField{offset:63, width: 1}, // 
};

// ページエントリ(Sv48)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageEntrySv48 {
    entry: u64,
}

impl PageEntrySv48 {
    pub const fn empty() -> PageEntrySv48 {
        PageEntrySv48 {entry:0}
    }
}

impl PageEntry for PageEntrySv48 {
    
    fn new() -> Self {
        PageEntrySv48 {entry:0}
    }

    // todo fix
    fn set_parmition(&mut self, flags :usize) {
        self.entry &= !SV48_ENTRY.r.pattern(1);
    }
    
    /* 当該ページ物理アドレスor次のテーブルのアドレスを設定する */
    fn set_ppn(&mut self, ppn :u64) {
        self.entry &= !SV48_ENTRY.ppn.pattern(0xffff_ffff_ffff_ffff); // PPN0クリア
        self.entry |= SV48_ENTRY.ppn.pattern(ppn);
    }

    /* 当該ページ物理アドレスor次のテーブルのアドレスを設定する */
    fn get_ppn(&self) -> u64 {
        SV48_ENTRY.ppn.mask(self.entry)
    }

    fn is_valid(&mut self) -> bool {
        SV48_ENTRY.v.mask(self.entry) == 1
    }

    /* 当該ページの有効化 */
    fn valid(&mut self) {
        self.entry |= SV48_ENTRY.v.pattern(1);
    }

    /* 当該ページの無効化 */
    fn invalid(&mut self) {
        self.entry &= !SV48_ENTRY.v.pattern(0xffff_ffff_ffff_ffff); // PPN0クリア
    }

    fn writable(&mut self) {
        self.entry |= SV48_ENTRY.w.pattern(1);
        self.entry |= SV48_ENTRY.r.pattern(1);
        self.entry |= SV48_ENTRY.x.pattern(1);
        //self.entry |= SV48_ENTRY.g.pattern(1);
    }


}

#[repr(C)]
#[derive(Clone, Copy)] //危険か？
#[repr(align(4096))]
pub struct PageTableSv48 {
    pub entry :[PageEntrySv48; 512],
}

impl PageTableSv48 {
    pub const fn empty() -> Self {
        PageTableSv48 {
            entry: [PageEntrySv48::empty(); 512]
            //entry: [0; 512]
        }
    }
}

impl PageTable for PageTableSv48 {
    type Entry = PageEntrySv48;

    // 初期化
    fn new() -> Self {
        PageTableSv48 { 
            entry: [PageEntrySv48::new(); 512]
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

