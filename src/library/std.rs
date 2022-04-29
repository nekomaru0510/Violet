//! stdライブラリ

use crate::PERIPHERALS;
use crate::environment::traits::serial::HasSerial;

use core::fmt::{self, Write};

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
    let mut serial= unsafe { PERIPHERALS.take_serial() };
    serial.write_fmt(args).unwrap();
    unsafe { PERIPHERALS.release_serial(serial) };
}



/*
/* リソース用トレイト */
use crate::resource::traits::tty::TraitTty;
use crate::resource::traits::timer::TraitTimerRs;

/* ライブラリ用トレイト */
use crate::library::traits::std::TraitStd;
*/

/*
#[derive(Clone)]
pub struct Std<T: TraitTty, U: TraitTimerRs> {
    tty: T,
    timer: U, 
}

impl<T, U> Std<T, U>
where
    T: TraitTty,
    U: TraitTimerRs,
{
    pub fn new(tty: T, timer: U) -> Self {
        Std { tty, timer, }
    }

    /*
    pub fn write(&self, c: u8) {
        self.tty.write(c);
    }*/
    pub fn write(&self, s: &str) {
        self.tty.write(s);
    }

    pub fn read(&self) -> u8 {
        self.tty.read()
    }
}

#[macro_export]
macro_rules! print{
     ($self:expr, $($arg:tt)*) => ($self.print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($self:expr, $fmt:expr) => (print!($self, concat!($fmt, "\n")));
    ($self:expr, $fmt:expr, $($arg:tt)*) => (print!($self, concat!($fmt, "\n"), $($arg)*));
}

impl<T, U> Std<T, U>
where
    T: TraitTty + core::fmt::Write,
    U: TraitTimerRs,
{
    pub fn print(&mut self, args: fmt::Arguments) {
        self.tty.write_fmt(args).unwrap();
    }
}

impl<T, U> TraitStd for Std<T, U>
where
    T: TraitTty + core::fmt::Write,
    U: TraitTimerRs,
{
    fn puts(&self, s: &str) {
        self.tty.write(s);
    }

    fn print(&mut self, args: fmt::Arguments) {
        self.tty.write_fmt(args).unwrap();
    }

    fn getc(&self) -> u8 {
        self.tty.read()
    }

    fn gettime(&self) -> u64 {
        self.timer.read()
    }

    fn settime(&self, t:u64) {
        self.timer.write(t);
    }

    fn set_alerm(&self, t: u64) {
        self.timer.set_interrupt_time(t);
    }

}
*/