use core::fmt::{self, Write};

pub trait StdoutIF {
    fn write_char(&self, c: u8) -> fmt::Result;
    fn write_str(&mut self, s: &str) -> fmt::Result;
}
