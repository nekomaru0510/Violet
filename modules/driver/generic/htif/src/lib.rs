//use core::ptr::{write_volatile, read_volatile};
#![no_std]

#[link_section = ".htif"]
#[no_mangle]
static mut tohost: u64 = 0;//0x80030000;
#[link_section = ".htif"]
#[no_mangle]
static mut fromhost: u64 = 0;

pub struct HTIF {
    console_buf: u64,
}

impl HTIF {
    pub fn new() -> Self {
        HTIF{console_buf: 0, }
    }

    pub fn write(&mut self, payload: [u64; 8]) {
        unsafe {
            self.set_tohost(0, 0, payload);
            loop {
                if fromhost != 0 {
                    break;
                    //if HTIF::fromhost_cmd() == 0
                }
            }
        }
    }

    pub fn read(&self) -> u64 {
        unsafe {
            fromhost
        }
    }

    pub fn do_tohost_fromhost(&mut self, dev: u64, cmd: u64, data: [u64; 8]) {
        unsafe {
            self.set_tohost(dev, cmd, data);

            loop {
                if fromhost != 0 {
                    if HTIF::fromhost_dev() == dev && HTIF::fromhost_cmd() == cmd {
                        fromhost = 0;
                        break;
                    }
                    self.check_fromhost();
                }
            }
        }
    }

    pub fn get_console_buf(&mut self) -> u64 {
        self.console_buf
    }

    //debug
    pub fn get_fromhost(&self) -> u64 {
        unsafe {
            fromhost
        }
    }

    fn set_tohost(&mut self, dev: u64, cmd: u64, data: [u64; 8]) {
        unsafe {
            // tohostが空になるまで待つ
            while tohost != 0 {
                self.check_fromhost();
            }
            tohost =  dev << 56 | cmd << 48 | (&data[0] as *const u64) as u64;
        }
    }

    // ダサい実装なので、あとで修正
    pub fn set_tohost_raw(&mut self, dev: u64, cmd: u64, data: u64) {
        unsafe {
            // tohostが空になるまで待つ
            while tohost != 0 {
                self.check_fromhost();
            }
            
            tohost =  dev << 56 | cmd << 48 | data;
        }
    }

    pub fn check_fromhost(&mut self) -> u64 {
        unsafe {
            if fromhost == 0 {
                return 0;
            }

            fromhost = 0;

            if HTIF::fromhost_cmd() == 0 {
                self.console_buf = 1 + HTIF::fromhost_data();
            }
            0
        }
    }

    fn fromhost_dev() -> u64 {
        unsafe {
            fromhost >> 56
        }
    }

    fn fromhost_cmd() -> u64 {
        unsafe {
            (fromhost << 8) >> 56
        }
    }

    fn fromhost_data() -> u64 {
        unsafe {
            (fromhost << 16) >> 16
        }
    }    
}