//mod driver::board::sifive_u::uart;
use crate::driver::board::sifive_u::uart::Uart;
use crate::driver::traits::serial::TraitSerial;
use crate::resource::io::serial::Serial;

pub struct Container {
    base: usize,
}

impl Container {
    pub fn new() -> Self {
        let uart = Uart::new(0x1001_0000);
        //uart.write('a' as u8);
        let serial = Serial::new(uart);
        serial.write('a' as u8);
        Container {base: 0x01,}
    }

}