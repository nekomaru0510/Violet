//! MMUのトレイト

pub trait TraitMmu {
    // Enable MMU
    fn enable_mmu();
    // Disable MMU
    //fn disable_mmu(&self);
}

// ページテーブル用トレイト
pub trait TraitPageTable {
    type Entry: TraitPageEntry;
    type Table: TraitPageTable;

    fn new() -> Self where Self: Sized;
    //fn get_entry(&mut self, vpn: u64) -> &mut <Self as PageTable>::Entry;
    fn get_entry(&mut self, vaddr: usize, table_level: usize) -> &mut <Self as TraitPageTable>::Entry;
    fn get_entry_ppn(&self, vpn: u64) -> u64;
    fn get_page_entry(&mut self, vaddr: usize) -> Option<&mut <Self as TraitPageTable>::Entry>;
    fn get_next_table(&self, vaddr: usize, idx: usize) -> Option<&mut <Self as TraitPageTable>::Table>;
    fn create_page_entry(&mut self, paddr: usize, vaddr: usize) -> Result<(), usize>;
    fn get_table(&mut self, vaddr: usize, idx: usize) -> Option<&mut <Self as TraitPageTable>::Table>;
    fn map_vaddr(&mut self, paddr: usize, vaddr: usize);
}

// ページエントリ用トレイト
pub trait TraitPageEntry {
    fn new() -> Self;
    fn set_paddr(&mut self, paddr: usize);
    fn set_ppn(&mut self, ppn: u64);
    fn get_ppn(&self) -> u64;
    fn is_valid(&mut self) -> bool;
    fn set_parmition(&mut self, flags: usize);
    fn valid(&mut self);
    fn invalid(&mut self);
    fn writable(&mut self);
    // Set page attribute
}


