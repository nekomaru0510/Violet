#![no_std]

use core::ptr::{write_volatile, read_volatile};
use crate::driver::traits::serial::TraitSerial;

pub struct Uart {
    base: usize,
}

const TXDATA: usize = 0x00;
const RXDATA: usize = 0x04;
/*
const TXCTRL: usize = 0x08;
const RXCTRL: usize = 0x0c;
const IE    : usize = 0x10;
const IP    : usize = 0x14;
const DIV   : usize = 0x1c;
*/

impl Uart {
    pub fn new(base: usize) -> Self {
        Uart {base: base,}
    }
}
impl TraitSerial for Uart {
    

    fn write(&self, c: u8) {
        unsafe {
            write_volatile((self.base + TXDATA) as *mut u8, c);
        }
    }

    /*pub*/ fn read(&self) -> u8 {
        unsafe {
            read_volatile((self.base + RXDATA) as *const u8)
        }
    }
}