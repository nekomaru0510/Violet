//! table module

//pub mod vshell;
//extern crate alloc;
//use alloc::boxed::Box;
//extern crate core;
//use core::cell::{RefCell, RefMut};

pub mod sig;

/*
pub mod sched;
pub mod dev;
*/

use crate::kernel::resource::Resource;

//use crate::kernel::rwlock::RwLock;

// todo delete
use crate::kernel::resource::io::serial::Serial;
use crate::kernel::resource::irq::Irq;
use crate::kernel::resource::handler_table::HandlerTable;

// 下のattributeは消したい
#[allow(improper_ctypes)]
extern "C" {
    static mut table: Table;
}

pub struct Table {
    pub resource: Resource,
}

impl Table {
    pub fn new() -> Self {
        Table {resource: Resource::new(), }
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
        &mut table.resource.io.serial
        //table.resource.io.serial2
    }

    pub unsafe fn get_mut_irq() -> &'static mut Irq {
        &mut table.resource.irq
    }

    pub unsafe fn get_mut_htable() -> &'static mut HandlerTable {
        &mut table.resource.htable
    }
}
/* 
Operator
table::get_mut_serial()
// mut or imut
// block or nonblock

Table
 */


