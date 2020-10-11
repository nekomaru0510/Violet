#![no_std]

use core::ptr::{write_volatile, read_volatile};
extern crate alloc;
use alloc::boxed::Box;

#[macro_use]
extern crate kernel;
//use kernel::driver_manager::*;

module_init!(init, Uart);

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

    pub fn write(&self, c: u8) {
        unsafe {
            write_volatile((self.base + TXDATA) as *mut u8, c);
        }
    }

    pub fn read(&self) -> u8 {
        unsafe {
            read_volatile((self.base + RXDATA) as *const u8)
        }
    }
}

fn init() -> Uart {
    let u = Uart::new(0x1001_0000);
    u.write('a' as u8);
    //let r = Box::new(Uart::new(0x1001_0000));
    u
    //r
}
