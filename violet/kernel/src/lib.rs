//! Kernel module
#![no_std]
#![feature(alloc_error_handler)]

pub mod minimum_allocator;
//pub mod table;
//pub mod rwlock; // テスト段階

//use table::Table;

pub struct Kernel {
    pub id: u32,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {id: 0, }
    }

    pub fn run(&mut self) {
        //unsafe {
            //Table::generate_table();
        //}
    }

}

