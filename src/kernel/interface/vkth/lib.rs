//! 

//use core::fmt::{self, Write};
use core::fmt;

use crate::kernel::operator::stdout::Stdout;
use crate::kernel::operator::stdin::Stdin;
use crate::kernel::operator::sig::Sig;

/* 与えられたフォーマット文字列と引数からcore::fmt::Argumentsを構築する */
#[macro_export]
macro_rules! print{
    ($($arg:tt)*) => (print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    let mut stdout = Stdout::new();
    stdout.write(args);
}

pub fn getc() -> u8 {
    let stdin = Stdin::new();
    stdin.read()
}


// todo delete
pub fn init_interrupt() {
    let mut sig = Sig::new();
    sig.enable(1);
}

// todo delete
pub fn register_timer_interrupt_handler(func: fn()) {
    let mut sig = Sig::new();
    sig.register(1, func);
}

