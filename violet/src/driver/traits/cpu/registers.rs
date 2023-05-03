//! 汎用レジスタ群のトレイト

pub trait TraitRegisters: Copy {
    fn switch(&mut self, regs: &mut Self);
}
