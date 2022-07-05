//! stdライブラリ

use crate::driver::traits::serial::TraitSerial;
use crate::environment::traits::serial::HasSerial;
use crate::PERIPHERALS;
use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

#[macro_export]
macro_rules! print{
     ($($arg:tt)*) => ($crate::library::std::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    let mut serial = unsafe { PERIPHERALS.take_serial() };
    serial.write_fmt(args).unwrap();
    unsafe { PERIPHERALS.release_serial(serial) };
}

pub fn getc() -> u8 {
    let serial = unsafe { PERIPHERALS.take_serial() };
    let res = serial.read();
    unsafe { PERIPHERALS.release_serial(serial) };
    res
}

pub fn memcpy(dst: usize, src: usize, size: usize) {
    for offset in 0..size {
        unsafe {
            let data = read_volatile((src + offset) as *const u8);
            write_volatile((dst + offset) as *mut u8, data);
        }
    }
}
