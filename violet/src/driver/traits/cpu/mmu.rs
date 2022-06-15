//! MMUのトレイト

pub trait TraitMmu {
    // MMU有効化
    fn enable_mmu(&self);
    // MMU無効化
    fn disable_mmu(&self);

    // ページエントリの取得
    //fn get_page_entry(&self, vaddr: usize);
    //fn get_page_entry(&mut self, vaddr: usize) -> &mut <Self as PageTable>::Entry;

}

// ページエントリ用トレイト
pub trait PageEntry {
    fn new() -> Self;
    fn set_ppn(&mut self, ppn: u64);
    fn get_ppn(&self) -> u64;
    fn is_valid(&mut self) -> bool;
    fn set_parmition(&mut self, flags: usize);
    fn valid(&mut self);
    fn invalid(&mut self);
    fn writable(&mut self);
}

// ページテーブル用トレイト
pub trait PageTable {
    type Entry: PageEntry;
    type Table: PageTable;

    fn new() -> Self;
    fn get_entry(&mut self, vpn: u64) -> &mut <Self as PageTable>::Entry;
    fn get_entry_ppn(&self, vpn: u64) -> u64;
    fn get_page_entry(&mut self, vaddr: usize) -> Option<&mut <Self as PageTable>::Entry>;
    fn get_next_table(&self, vaddr: usize, idx: usize) -> Option<&mut <Self as PageTable>::Table>;
}