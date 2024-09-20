//! Device access logger

use super::VirtualDeviceT;
use super::{read_raw, write_raw};
use crate::library::std::get_tick;
use crate::utils::logging::{LogManager};

enum AccessType {
    Read(ReadAccessLog),
    Write(WriteAccessLog),
    Interrupt(InterruptAccessLog),
}

struct ReadAccessLog {
    addr: usize,
    val: usize,
}

struct WriteAccessLog {
    addr: usize,
    val: usize,
}

struct InterruptAccessLog {
    intid: usize,
}

#[repr(C)]
#[repr(align(4096))]
pub struct AccessLogger {
    path_through: bool,
    log: LogManager<AccessType>,
}

impl AccessLogger {
    pub const fn new(path_through: bool) -> Self {
        AccessLogger {
            path_through,
            log: LogManager::new(get_tick),
        }
    }
}


impl VirtualDeviceT for AccessLogger {
    fn read(&mut self, addr: usize) -> usize {
        if self.path_through {
            let val = read_raw(addr);
            self.log.log(AccessType::Read(ReadAccessLog { addr, val }));
            val
        } else {
            self.log.log(AccessType::Read(ReadAccessLog { addr, val: 0 }));
            0
        }
    }

    fn write(&mut self, addr: usize, val: usize) {
        self.log.log(AccessType::Write(WriteAccessLog { addr, val }));
        if self.path_through {
            return write_raw(addr, val)
        }
    }

    fn interrupt(&mut self, intid: usize) {
        self.log.log(AccessType::Interrupt(InterruptAccessLog { intid }));
    }
}