//! シリアルデバイス用のトレイト

use core::fmt::{self, Write};

pub trait TraitSerial: Write {
    fn write(&self, c: u8);
    fn read(&self) -> u8;
    fn enable_interrupt(&self);
    fn disable_interrupt(&self);
    fn write_str(&mut self, s: &str) -> fmt::Result;
}
