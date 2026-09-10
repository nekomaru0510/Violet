use core::fmt::{self, Write};
use crate::uart16550::Uart;

pub fn _print(args: fmt::Arguments) {
    unsafe {
        match UART {
            Some(ref mut uart) => uart.write_fmt(args).unwrap(),
            None => {
                UART = Some(Uart::new(
                    UART_BASE, 0xbebc200, 115200, 0x2, 0x4
                ));
                if let Some(ref mut uart) = UART {
                    uart.init();
                }
                _print(args);
            }
            
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[link_section = ".data"]
static mut UART: Option<Uart> = None;

static UART_BASE: usize = 0x5090_0000;