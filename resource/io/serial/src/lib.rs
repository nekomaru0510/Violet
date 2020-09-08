#![no_std]

use core::fmt::{self, Write};
use uart::Uart;

pub struct Serial {
    uart: Uart,
}

impl Serial {
    pub fn new() -> Self {
        Serial{uart: Uart::new(0x1001_0000),}
    }

    pub fn write(&self, c:u8) {
        self.uart.write(c);
    }

    pub fn read(&self) -> u8 {
        self.uart.read()
    }

}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.write(c);
        }
        Ok(())
    }
}

