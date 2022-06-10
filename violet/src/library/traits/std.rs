//! stdライブラリ用のトレイト

use core::fmt;

pub trait TraitStd {
    fn puts(&self, s: &str);
    fn print(&mut self, args: fmt::Arguments);
    fn getc(&self) -> u8;
    fn gettime(&self) -> u64;
    fn settime(&self, t:u64);
    fn set_alerm(&self, t: u64);
}