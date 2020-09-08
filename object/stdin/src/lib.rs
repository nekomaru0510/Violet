//! stdin module
//! 
#![no_std]

//use table::Table;
use serial::Serial;

/*
pub struct Stdin<'a> {
    serial: &'a Serial,
}

impl<'a> Stdin<'a> {
    pub fn new() -> Self {
        unsafe {
            Stdin {serial: &Table::table().serial}
        }
    }
    pub fn read(&self) -> u8 {
        self.serial.read()
    }
}
*/

pub struct Stdin {
    serial: Serial,
}

impl Stdin {
    pub fn new() -> Self {
        Stdin {serial: Serial::new()}
    }
    
    pub fn read(&self) -> u8 {
        self.serial.read()
    }
}

