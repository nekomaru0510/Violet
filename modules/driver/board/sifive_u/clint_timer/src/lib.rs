//! Sifive Core-Local Interruptor(CLINT)'s timer module
#![no_std]

use core::ptr::{write_volatile, read_volatile};
//use interface::driver::TimerAttr;
use class::timer::TimerAttr;

pub struct ClintTimer {
    base: usize, /* 0x0200_4000 */
}

const MTIMECMP0: usize = 0x0;
const MTIME0: usize = 0x7ff8;

impl TimerAttr for ClintTimer {
    fn new() -> Self {
        ClintTimer {base: 0x0200_4000,}

    }
    /* pub fn new(base: usize) -> Self {
        ClintTimer {base: base,}

    } */

    fn write(&self, t: u64) {
        self.write_mtime(t);
    }

    fn read(&self) -> u64 {
        self.read_mtime()
    }

    fn enable_interrupt(&self) {
        // nothing to do
    }

    fn disable_interrupt(&self) {
        self.write_mtimecmp(0xffff_ffff_ffff_ffff);
    }

    fn set_interrupt_time(&self, t: u64) {
        self.write_mtimecmp(t);
    }

}

impl ClintTimer {
    pub fn write_mtimecmp(&self, t: u64) {
        unsafe {
            write_volatile((self.base + MTIMECMP0) as *mut u64, t);
        }
    }

    pub fn read_mtimecmp(&self) -> u64 {
        unsafe {
            read_volatile((self.base + MTIMECMP0) as *const u64)
        }
    }

    pub fn write_mtime(&self, t: u64) {
        unsafe {
            write_volatile((self.base + MTIME0) as *mut u64, t);
        }
    }

    pub fn read_mtime(&self) -> u64 {
        unsafe {
            read_volatile((self.base + MTIME0) as *const u64)
        }
    }
}