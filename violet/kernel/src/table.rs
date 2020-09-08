//! table module

#![no_std]

// todo delete
use serial::Serial;
use irq::Irq;
use cpu::Cpu;
pub mod handler_table;
use handler_table::HandlerTable;

// 下のattributeは消したい
#[allow(improper_ctypes)]
extern "C" {
    static mut table: Table;
}

pub struct Table {
    pub cpu: Cpu,
    pub irq: Irq,
    pub serial: Serial,
    pub htable: HandlerTable,
}

impl Table {
    pub fn new() -> Self {
        Table {cpu: Cpu::new(), irq: Irq::new(), serial: Serial::new(), htable: HandlerTable::new(), }
    }

    pub unsafe fn generate_table() {
        //table = &'static mut Box::new(Table::new());
        //table = &mut Box::new(Table::new());
        table = Table::new();
    }

    // このままの実装では、所有権や対マルチスレッド機能を利用できていない。
    // ちゃんと考える。
    // あと、一々関数呼出しが入ると遅くなる気がするので、
    // 性能面でも考える
    pub unsafe fn table() -> &'static mut Table {
        &mut table
    }

    // できれば、返す型もdynとかで動的に生成させたい
    //pub unsafe fn get_mut_serial() -> /* RefMut<'static, Serial> */ RwLock<Serial> {
    //pub unsafe fn get_mut_serial() -> Result<&'static RwLock<T>, &'static str> {
    pub unsafe fn get_mut_serial() -> &'static mut Serial {
        //&mut table.resource.io.serial
        //&mut table.resource.io.serial2.write().unwrap()
        //let mut a = table.resource.io.serial2.write().unwrap();
        //&mut *table.resource.io.serial2.write().unwrap()
        //&mut *a
        //*a += 1;
        &mut table.serial
        //table.resource.io.serial2
    }

    pub unsafe fn get_mut_irq() -> &'static mut Irq {
        &mut table.irq
    }

    pub unsafe fn get_mut_htable() -> &'static mut HandlerTable {
        &mut table.htable
    }
}
/* 
Operator
table::get_mut_serial()
// mut or imut
// block or nonblock

Table
 */


