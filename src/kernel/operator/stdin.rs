//! stdin module
//! 

use crate::kernel::table::Table;
use crate::kernel::resource::io::serial::Serial;

pub struct Stdin<'a> {
    serial: &'a Serial,
}

impl<'a> Stdin<'a> {
    pub fn new() -> Self {
        unsafe {
            Stdin {serial: &Table::table().resource.io.serial}
        }
    }
    pub fn read(&self) -> u8 {
        self.serial.read()
    }
}




