
use crate::driver::traits::serial::TraitSerial;
use core::fmt::{self, Write};


/*
pub struct Serial {
    uart: Uart,
}
*/
pub struct Serial<T: TraitSerial> {
    uart: T,
}

impl<T> Serial<T> 
    where
        T: TraitSerial,
{
    pub fn new(uart: T) -> Self {
        Serial{uart,}
    }

    pub fn write(&self, c:u8) {
        self.uart.write(c);
    }

    pub fn read(&self) -> u8 {
        self.uart.read()
    }

}

/*
impl Write for Serial<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.write(c);
        }
        Ok(())
    }
}
*/