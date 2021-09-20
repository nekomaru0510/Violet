//! stdライブラリ用のトレイト

use core::fmt;

pub trait TraitStd {
    fn puts(&self, s: &str);
    fn print(&mut self, args: fmt::Arguments);
    fn getc(&self) -> u8;
}