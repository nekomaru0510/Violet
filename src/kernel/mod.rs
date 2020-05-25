//! Kernel module

pub mod minimum_allocator;
//pub mod rwlock; // テスト段階

#[macro_use]
pub mod interface;
pub mod operator;
pub mod resource;
pub mod driver;
pub mod table;

//use interface::Interface;
//use resource::Resource;
//use operator::Operator;
use table::Table;

pub struct Kernel {
    pub id: u32,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {id: 0, }
    }

    pub fn run(&mut self) {
        unsafe {
            Table::generate_table();
        }
    }

}

