#![no_std]

pub struct Uart {
    fesyscall: FeSyscall,
}

impl Uart {
    pub fn new(base: usize) -> Self {
        Uart {fesyscall: FeSyscall::new(),}
    }

    pub fn write(&self, c: u8) {
        //self.sys_write(1, &("hello".as_bytes())[0] as *const u8 , 5);
        self.fesyscall.sys_write(1, &c as *const u8 , 1);
    }

    pub fn read(&self) -> u8 {
        let mut buf: [u8; 32] = [0; 32];
        self.fesyscall.sys_read(0, &buf[0] as *const u8 , 1);
        if buf[0] == 0x0a {
               buf[0] = 0x0d;
        }
        buf[0]
    }
}

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

    pub fn write(&self, payload: [u64; 8]) {
        unsafe {
            self.set_tohost(0, 0, payload);
            loop {
                if fromhost != 0 {
                    break;
                }
            }
        }
    }

    pub fn read(&self) -> u64 {
        unsafe {
            fromhost
        }
    }

    pub fn do_tohost_fromhost(&self, dev: u64, cmd: u64, data: [u64; 8]) {
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

    pub fn get_console_buf(&self) -> u64 {
        self.console_buf
    }

    //debug
    pub fn get_fromhost(&self) -> u64 {
        unsafe {
            fromhost
        }
    }

    fn set_tohost(&self, dev: u64, cmd: u64, data: [u64; 8]) {
        unsafe {
            // tohostが空になるまで待つ
            while tohost != 0 {
                self.check_fromhost();
            }
            tohost =  dev << 56 | cmd << 48 | (&data[0] as *const u64) as u64;
        }
    }

    // ダサい実装なので、あとで修正
    pub fn set_tohost_raw(&self, dev: u64, cmd: u64, data: u64) {
        unsafe {
            // tohostが空になるまで待つ
            while tohost != 0 {
                self.check_fromhost();
            }
            
            tohost =  dev << 56 | cmd << 48 | data;
        }
    }

    pub fn check_fromhost(&self) -> u64 {
        unsafe {
            if fromhost == 0 {
                return 0;
            }

            fromhost = 0;

            if HTIF::fromhost_cmd() == 0 {
                //self.console_buf = 1 + HTIF::fromhost_data();
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

    #[allow(dead_code)]
    fn fromhost_data() -> u64 {
        unsafe {
            (fromhost << 16) >> 16
        }
    }    
}


enum SyscallId {
    SysRead = 63,
    SysWrite = 64,
    SysExit = 93,
}

pub struct FeSyscall {
    htif: HTIF,
}

impl FeSyscall {
    pub fn new() -> Self {
        FeSyscall{htif: HTIF::new(), }
    }

    pub fn get_console_buf(&self) -> u64 {
        //self.htif.get_console_buf()
        //self.htif.get_fromhost()

        self.htif.check_fromhost();
        
        let ch = self.htif.get_console_buf();
        if ch >= 0 {
            self.htif.set_tohost_raw(1, 0, 0);
        }
        return ch as u64 - 1;
    }

    pub fn sys_write(&self, fd: u64, buf: *const u8 , size: u64) {
        let payload = [SyscallId::SysWrite as u64, fd, buf as u64, size, 0, 0, 0, 0];
        self.htif.do_tohost_fromhost(0, 0, payload);
    }

    pub fn sys_read(&self, fd: u64, buf: *const u8 , size: u64) {
        let payload = [SyscallId::SysRead as u64, fd, buf as u64, size, 0, 0, 0, 0];
        self.htif.do_tohost_fromhost(0, 0, payload);
    }

    pub fn sys_exit(&self) {
        let payload = [SyscallId::SysExit as u64, 0, 0, 0, 0, 0, 0, 0];
        self.htif.write(payload);
    }

}

