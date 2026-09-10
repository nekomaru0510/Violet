//! ns16550 UART driver

use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

#[derive(Clone)]
pub struct Uart {
    base: usize,
    clock_freq: usize,
    baudrate: usize,
    reg_shift: usize,
    reg_io_width: usize,
}

const RHR_OFFSET: usize = 0x00;
const THR_OFFSET: usize = 0x00;
const DLL_OFFSET: usize = 0x00;
const IER_OFFSET: usize = 0x01;
const DLM_OFFSET: usize = 0x01;
const FCR_OFFSET: usize = 0x02;
const IIR_OFFSET: usize = 0x02;
const LCR_OFFSET: usize = 0x03;
const MCR_OFFSET: usize = 0x04;
const LSR_OFFSET: usize = 0x05;
const MSR_OFFSET: usize = 0x06;
const SCR_OFFSET: usize = 0x07;
const MDR1_OFFSET: usize = 0x08;

const LSR_THRE: u8 = 0x20;
const LSR_DR: u8 = 0x01;

impl Uart {
    pub fn new(base: usize, clock_freq: usize, baudrate: usize, reg_shift: usize, reg_io_width: usize) -> Self {
        Uart { base, clock_freq, baudrate, reg_shift, reg_io_width }
    }

    pub fn init(&self) {
        let divisor = (self.clock_freq + 8 * self.baudrate) / (16 * self.baudrate);
        self.reg_write(LCR_OFFSET, 0x80);
        self.reg_write(DLL_OFFSET, (divisor & 0xff) as u8);
        self.reg_write(DLM_OFFSET, ((divisor >> 8) & 0xff) as u8);
        self.reg_write(LCR_OFFSET, 0x03);
        self.reg_write(FCR_OFFSET, 0x01);
        self.reg_write(MCR_OFFSET, 0x00);
        self.reg_write(IER_OFFSET, 0x00); // Disable interrupts
        self.reg_read(LSR_OFFSET);
        self.reg_read(RHR_OFFSET);
        self.reg_read(SCR_OFFSET);
    }

    fn reg_write(&self, offset: usize, value: u8) {
        let addr = self.base + (offset << self.reg_shift);
        match self.reg_io_width {
            1 => unsafe { write_volatile(addr as *mut u8, value) },
            2 => unsafe { write_volatile(addr as *mut u16, value as u16) },
            4 => unsafe { write_volatile(addr as *mut u32, value as u32) },
            _ => panic!("Invalid reg_io_width"),
        }
    }

    fn reg_read(&self, offset: usize) -> u8 {
        let addr = self.base + (offset << self.reg_shift);
        match self.reg_io_width {
            1 => unsafe { read_volatile(addr as *const u8) },
            2 => unsafe { read_volatile(addr as *const u16) as u8 },
            4 => unsafe { read_volatile(addr as *const u32) as u8 },
            _ => panic!("Invalid reg_io_width"),
        }
    }
       
}

impl Uart {
    fn write(&self, c: u8) {
        while self.reg_read(LSR_OFFSET) & LSR_THRE == 0 {}
        self.reg_write(THR_OFFSET, c);
    }
}

impl Write for Uart {
   

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            if c == b'\n' {
                self.write(b'\r');
            }
            self.write(c);
        }
        Ok(())
    }
}
