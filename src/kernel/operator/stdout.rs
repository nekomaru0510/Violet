//! stdout module

use core::fmt::{self, Write};

use crate::kernel::table::Table;
use crate::kernel::resource::io::serial::Serial;

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

