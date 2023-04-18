//! stdライブラリ

use crate::driver::traits::serial::TraitSerial;
//use crate::PERIPHERALS;
use core::fmt::{self};
use core::ptr::{read_volatile, write_volatile};
use crate::kernel::container::*;

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
    let con = current_mut_container();
    match &mut con.unwrap().serial {
        None => (),
        Some(s) => s.write_fmt(args).unwrap(),
    }
}

pub fn getc() -> u8 {
    let con = current_mut_container();
    match &mut con.unwrap().serial {
        None => 0,
        Some(s) => s.read(),
    }
}

pub fn memcpy(dst: usize, src: usize, size: usize) {
    for offset in 0..size {
        unsafe {
            let data = read_volatile((src + offset) as *const u8);
            write_volatile((dst + offset) as *mut u8, data);
        }
    }
}
