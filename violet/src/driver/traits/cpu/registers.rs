//! 汎用レジスタ群のトレイト

pub trait TraitRegisters: Copy {
    fn switch(&mut self, regs: &mut Self);
    /*
    fn set(&mut self, idx: usize, );
    fn get(&self, idx: usize) -> ;
    */
}
