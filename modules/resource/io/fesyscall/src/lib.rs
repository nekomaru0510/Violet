/// frontend syscall
#![no_std]

use htif::HTIF;

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

    pub fn get_console_buf(&mut self) -> u64 {
        //self.htif.get_console_buf()
        //self.htif.get_fromhost()

        self.htif.check_fromhost();
        
        let mut ch  = self.htif.get_console_buf();
        if ch >= 0 {
            self.htif.set_tohost_raw(1, 0, 0);
        }
        return ch as u64 - 1;
    }

    pub fn sys_write(&mut self, fd: u64, buf: *const u8 , size: u64) {
        let payload = [SyscallId::SysWrite as u64, fd, buf as u64, size, 0, 0, 0, 0];
        //self.htif.write(payload);
        self.htif.do_tohost_fromhost(0, 0, payload);
        //while self.htif.read() != 0 {}
    }

    pub fn sys_read(&mut self, fd: u64, buf: *const u8 , size: u64) {
        let payload = [SyscallId::SysRead as u64, fd, buf as u64, size, 0, 0, 0, 0];
        self.htif.do_tohost_fromhost(0, 0, payload);
        //self.htif.write(payload);
        //self.htif.read();
    }

    pub fn sys_exit(&mut self) {
        let payload = [SyscallId::SysExit as u64, 0, 0, 0, 0, 0, 0, 0];
        self.htif.write(payload);
    }

}

