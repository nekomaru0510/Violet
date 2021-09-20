//! stdライブラリ

use core::fmt::{self, Write};

/* リソース用トレイト */
use crate::resource::traits::tty::TraitTty;

/* ライブラリ用トレイト */
use crate::library::traits::std::TraitStd;

pub struct Std<T: TraitTty> {
    tty: T,
}

impl<T> Std<T>
where
    T: TraitTty,
{
    pub fn new(tty: T) -> Self {
        Std { tty }
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

impl<T> Std<T>
where
    T: TraitTty + core::fmt::Write,
{
    pub fn print(&mut self, args: fmt::Arguments) {
        self.tty.write_fmt(args).unwrap();
    }

    pub fn getc(&self) -> u8 {
        self.tty.read()
    }
}

impl<T> TraitStd for Std<T>
where
    T: TraitTty + core::fmt::Write,
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
}
