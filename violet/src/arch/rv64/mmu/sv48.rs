//! Page entry and page table for Sv48

extern crate core;
use core::intrinsics::transmute;

use crate::library::bitfield::BitField;
use crate::arch::rv64::mmu::{get_new_page_table_idx, get_page_table_addr};
use crate::arch::traits::mmu::{TraitPageEntry, TraitPageTable};

const PAGE_TABLE_LEVEL: usize = 4;      // Number of page table levels
const NUM_OF_PAGE_ENTRY: usize = 512;   // Number of page entries per table

// Virtual address bit field
pub struct VirtualAddressFieldSv48 {
    pub page_offset: BitField,
    pub vpn: [BitField; 4],
}

// Physical address bit field
pub struct PhysicalAddressFieldSv48 {
    pub page_offset: BitField,
    pub ppn: [BitField; 4],
    pub ppn_all: BitField,
}

// Page entry bit field
pub struct PageEntryFieldSv48 {
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
    pub ppn: BitField,  //
    pub pbmt: BitField, //
    pub n: BitField,    //
}

pub const SV48_VA: VirtualAddressFieldSv48 = VirtualAddressFieldSv48 {
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
        BitField {
            offset: 39,
            width: 9,
        },
    ],
};
pub const SV48_PA: PhysicalAddressFieldSv48 = PhysicalAddressFieldSv48 {
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
            width: 9,
        },
        BitField {
            offset: 39,
            width: 17,
        },
    ],
    ppn_all: BitField {
        offset: 12,
        width: 44,
    },
};
pub const SV48_ENTRY: PageEntryFieldSv48 = PageEntryFieldSv48 {
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
    pbmt: BitField {
        offset: 61,
        width: 2,
    },
    n: BitField {
        offset: 63,
        width: 1,
    },
};

fn vaddr_to_vpn(vaddr: u64, idx: usize) -> u64 {
    SV48_VA.vpn[idx].mask(vaddr) as u64
}

fn paddr_to_ppn(paddr: u64) -> u64 {
    SV48_PA.ppn_all.mask(paddr) as u64
}

// Page entry(Sv48)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageEntrySv48 {
    entry: u64,
}

impl PageEntrySv48 {
    pub const fn empty() -> PageEntrySv48 {
        PageEntrySv48 { entry: 0 }
    }
}

impl TraitPageEntry for PageEntrySv48 {
    fn new() -> Self {
        PageEntrySv48 { entry: 0 }
    }

    // todo fix
    fn set_parmition(&mut self, flags: usize) {
        self.entry &= !SV48_ENTRY.r.pattern(1);
    }

    fn set_paddr(&mut self, paddr: usize) {
        self.set_ppn(paddr_to_ppn(paddr as u64));
    }

    // Set the physical address of the page or the address of the next table
    fn set_ppn(&mut self, ppn: u64) {
        self.entry &= !SV48_ENTRY.ppn.pattern(0xffff_ffff_ffff_ffff); // Clear PPN0
        self.entry |= SV48_ENTRY.ppn.pattern(ppn);
    }

    // Get the physical address of the page or the address of the next table
    fn get_ppn(&self) -> u64 {
        SV48_ENTRY.ppn.mask(self.entry)
    }

    fn is_valid(&mut self) -> bool {
        SV48_ENTRY.v.mask(self.entry) == 1
    }

    fn valid(&mut self) {
        self.entry |= SV48_ENTRY.v.pattern(1);
    }

    fn invalid(&mut self) {
        self.entry &= !SV48_ENTRY.v.pattern(0xffff_ffff_ffff_ffff); // Clear PPN0
    }

    fn writable(&mut self) {
        self.entry |= SV48_ENTRY.w.pattern(1);
        self.entry |= SV48_ENTRY.r.pattern(1);
        self.entry |= SV48_ENTRY.x.pattern(1);
        self.entry |= SV48_ENTRY.u.pattern(1); //test
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[repr(align(16384))]
pub struct PageTableSv48 {
    pub entry: [PageEntrySv48; NUM_OF_PAGE_ENTRY],
}

impl PageTableSv48 {
    pub const fn empty() -> Self {
        PageTableSv48 {
            entry: [PageEntrySv48::empty(); 512],
        }
    }
}

impl TraitPageTable for PageTableSv48 {
    type Entry = PageEntrySv48;
    type Table = PageTableSv48;

    fn new() -> Self {
        PageTableSv48 {
            entry: [PageEntrySv48::new(); NUM_OF_PAGE_ENTRY],
        }
    }

    fn get_entry(&mut self, vaddr: usize, table_level: usize) -> &mut <Self as TraitPageTable>::Entry {
        let idx = PAGE_TABLE_LEVEL - table_level;
        &mut self.entry[vaddr_to_vpn(vaddr as u64, idx) as usize]
    }

    fn get_entry_ppn(&self, vpn: u64) -> u64 {
        let e = self.entry[vpn as usize];
        e.get_ppn()
    }

    fn get_page_entry(&mut self, vaddr: usize) -> Option<&mut <Self as TraitPageTable>::Entry> {
        let vpn = SV48_VA.vpn[0].mask(vaddr as u64);
        let mut table: &mut PageTableSv48 = self;

        for i in (1..PAGE_TABLE_LEVEL).rev() {
            // Get next table
            match (*table).get_next_table(vaddr, i) {
                None => return None,
                Some(t) => table = t,
            }
        }
        Some(&mut ((*table).entry[vpn as usize]))
    }

    fn get_next_table(&self, vaddr: usize, idx: usize) -> Option<&mut <Self as TraitPageTable>::Table> {
        let vpn = SV48_VA.vpn[idx].mask(vaddr as u64);
        let ret = (*self).get_entry_ppn(vpn) << 12;
        if ret == 0 {
            return None;
        }
        let t: &mut <Self as TraitPageTable>::Table = unsafe { transmute(ret) };
        Some(t)
    }

    // Create a page entry
    // If the intermediate page table does not exist, return the number of stages as an error
    fn create_page_entry(&mut self, paddr: usize, vaddr: usize) -> Result<(), usize> {
        let mut table: &mut PageTableSv48 = self;
        for i in (1..PAGE_TABLE_LEVEL).rev() {
            match (*table).get_next_table(vaddr, i) {
                None => {
                    return Err(PAGE_TABLE_LEVEL - i);
                }
                Some(t) => table = t,
            }
        }
        (*table)
            .get_entry(vaddr, 4)
            .set_ppn(paddr_to_ppn(paddr as u64));
        (*table).get_entry(vaddr, 4).valid();
        (*table).get_entry(vaddr, 4).writable();
        Ok(())
    }

    // Get the specified page table
    fn get_table(&mut self, vaddr: usize, idx: usize) -> Option<&mut <Self as TraitPageTable>::Table> {
        let vpn = SV48_VA.vpn[0].mask(vaddr as u64);
        let mut table: &mut PageTableSv48 = self;
        let idx = idx - 1;

        for i in ((PAGE_TABLE_LEVEL - idx)..PAGE_TABLE_LEVEL).rev() {
            // Get next table
            match (*table).get_next_table(vaddr, i) {
                None => return None,
                Some(t) => table = t,
            }
        }
        Some(table)
    }

    fn map_vaddr(&mut self, paddr: usize, vaddr: usize) {
        for idx in 1..PAGE_TABLE_LEVEL {
            match self.create_page_entry(paddr, vaddr) {
                Ok(()) => break,
                Err(i) => unsafe {
                    match self.get_table(vaddr, i) {
                        None => return,
                        Some(t) => {
                            t.get_entry(vaddr, i)
                                .set_paddr(transmute(get_page_table_addr(get_new_page_table_idx())));
                            t.get_entry(vaddr, i).valid();
                        }
                    }
                },
            }
        }
    }
    
    /* [todo impl] */    
    /*fn map_vaddr_size(&mut self, paddr: usize, vaddr: usize, size: usize) {
        self.map_vaddr(paddr, vaddr);
    }*/

    fn v2p(&mut self, vaddr: usize) -> usize {
        match self.get_page_entry(vaddr) {
            None => 0,
            Some(e) => ((e.get_ppn() << 12) as usize) | (vaddr & 0x0fff),
        }   
    }
  
}

impl PageTableSv48 {
    
}
