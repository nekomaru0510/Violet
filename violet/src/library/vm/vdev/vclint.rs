//! Virtual CLINT

use super::VirtualDeviceT;
use super::read_raw;

use crate::arch::rv64::sbi;
use crate::arch::rv64::extension::hypervisor::Hext;
use crate::arch::rv64::trap::int::Interrupt;

#[repr(C)]
#[repr(align(4096))]
pub struct VClint {
    mtime: u64,
    mtimecmp: u64,
}

const MTIME_OFFSET: usize = 0xbff8;
const MTIMECMP_OFFSET: usize = 0x4000;

const BASE_ADDRESS: usize = 0x200_0000; /* [todo delete] */
const ADDRESS_RANGE: usize = 0x1_0000;
const MASK: usize = 0xffff;

impl VClint {
    pub const fn new() -> Self {
        VClint {
            mtime: 0,
            mtimecmp: 0,
        }
    }

    fn mtime_write(&mut self, addr: usize, val: u64) {
        ()
    }

    fn mtime_read(&mut self, addr: usize) -> u64 {
        self.mtime = self.mtime + 1; // Add time every time it is referenced
        return self.mtime
    }
    
    fn mtimecmp_write(&mut self, addr: usize, val: u64) {
        // Flush guest timer interrupt
        Hext::flush_vsmode_interrupt(Interrupt::bit(
            Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
        ));
        
        // If an interrupt is generated with the relative time requested by the Guest OS, an infinite loop will occur due to the interrupt.
        // This is because the processing time due to virtualization overhead is longer than the tick of the guest OS.
        // Therefore, get the time when the interrupt occurs and generate the interrupt based on that time.
        // # 3000 is an arbitrary value, and the actual value depends on the execution environment.
        let current = u64::from_be(read_raw::<u64>(0x0200_bff8)) + 3000;
        sbi::sbi_set_timer((current) as u64);

        self.mtimecmp = val;
        self.mtime = (current) as u64;
    }

    fn mtimecmp_read(&mut self, addr: usize) -> u64 {
        return self.mtimecmp
    }
}

impl VirtualDeviceT for VClint {
    fn write(&mut self, addr: usize, val: usize) {
        // [todo fix] Consolidate register acquisition into a function
        match addr & MASK {
            MTIME_OFFSET => self.mtime_write(addr, val as u64),
            MTIMECMP_OFFSET => self.mtimecmp_write(addr, val as u64),
            _ => (),
        };
    }

    fn read(&mut self, addr: usize) -> usize {
        let ret = match addr & MASK {
            MTIME_OFFSET => self.mtime_read(addr),
            MTIMECMP_OFFSET => self.mtimecmp_read(addr),
            _ => 0,
        };
        ret as usize
    }

    fn interrupt(&mut self, intid: usize) {
        ()
    }
}
