//! Kernel module
#![no_std]
#![feature(alloc_error_handler)]

#![feature(const_raw_ptr_to_usize_cast)]

pub mod minimum_allocator;
pub mod driver_manager;
use driver_manager::DriverManager;
//pub mod table;
//pub mod rwlock; // テスト段階

//use table::Table;

pub struct Kernel {
    pub id: u32,
}

impl Kernel {
    pub fn new() -> Self {
        let drvmgr = DriverManager::new();
        drvmgr.call_initializer();
        Kernel {id: 0, }
    }

    pub fn run(&mut self) {
        //unsafe {
            //Table::generate_table();
        //}
    }

}

