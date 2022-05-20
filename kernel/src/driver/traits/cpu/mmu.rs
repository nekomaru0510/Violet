//! MMUのトレイト

pub trait TraitMmu {
    // MMU有効化
    fn enable_mmu(&self);
    // MMU無効化
    fn disable_mmu(&self);
}
