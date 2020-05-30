//! sig module

use crate::kernel::table::Table;
use crate::kernel::resource::irq::Irq;
use crate::kernel::resource::handler_table::HandlerTable;

pub struct Sig<'a> {
    irq: &'a mut Irq,
    table: &'a mut HandlerTable,
}

impl<'a> Sig<'a> {
    pub fn new() -> Self {
        unsafe {
            Sig {irq: Table::get_mut_irq(), table: Table::get_mut_htable(), }
        }
    }

    pub fn enable(&mut self, id: u64) {
        self.irq.enable(id);
    }
    
    // 本当は抽象化された割込み番号を利用したい。
    pub fn register(&mut self, id: u32, func: fn()) {
        self.table.register(id, func);
    }

    pub fn call(&self, id: u32) {
        self.table.call(id);
    }

}
