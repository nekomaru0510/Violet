//! stdout module
#![no_std]

use core::fmt::{self, Write};

//use table::Table;
use serial::Serial;

/*
pub struct Stdout<'a> {
    serial: &'a mut Serial,
}

impl<'a> Stdout<'a> {
    pub fn new() -> Self {
        unsafe {
            Stdout {serial: Table::get_mut_serial()}
        }
    }
    pub fn write(&mut self, args: fmt::Arguments) {
        self.serial.write_fmt(args).unwrap();
    }
}
*/

pub struct Stdout {
    serial: Serial,
}

impl Stdout {
    pub fn new() -> Self {
        Stdout {serial: Serial::new()}
    }
    
    pub fn write(&mut self, args: fmt::Arguments) {
        self.serial.write_fmt(args).unwrap();
    }
}


