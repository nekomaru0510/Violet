//! 汎用レジスタ群のトレイト

pub trait TraitRegisters: Copy {
    // コンテキストの退避
    fn save_from(&self, from: &Self);
    // コンテキストの復帰
    fn restore_to(&self, to: &Self);
}
