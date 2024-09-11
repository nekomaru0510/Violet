//! Standard library functions

use crate::resource::{
    get_mut_resources, get_resources, BorrowMutResource, BorrowResource, ResourceType,
};
use core::fmt::{self};
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
    if let BorrowMutResource::Serial(s) = get_mut_resources().get_mut(ResourceType::Serial, 0) {
        s.write_fmt(args).unwrap();
    }
}

pub fn getc() -> u8 {
    if let BorrowResource::Serial(s) = get_resources().get(ResourceType::Serial, 0) {
        s.read()
    } else {
        0
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
