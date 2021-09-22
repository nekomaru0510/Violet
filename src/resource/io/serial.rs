//! シリアルリソース

use core::fmt::{self, Write};

/* リソース用トレイト */
use crate::resource::traits::tty::TraitTty;

/* ドライバ用トレイト */
use crate::driver::traits::serial::TraitSerial;

#[derive(Clone)]
pub struct Serial<T: TraitSerial> {
    uart: T,
}

impl<T> Serial<T>
where
    T: TraitSerial,
{
    pub fn new(uart: T) -> Self {
        Serial { uart }
    }
}

impl<T> TraitTty for Serial<T>
where
    T: TraitSerial,
{
    /*
    fn write(&self, c: u8) {
        self.uart.write(c);
    }
    */
    fn write(&self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.uart.write(c);
        }
        Ok(())
    }

    fn read(&self) -> u8 {
        self.uart.read()
    }
}

impl<T> Write for Serial<T>
where
    T: TraitSerial,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.uart.write(c);
        }
        Ok(())
    }
}
