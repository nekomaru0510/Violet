//! 仮想CLINT

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
const MASK: usize = 0xffff; /* [todo fix] 上記要素から算出できるように */

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
        self.mtime = self.mtime + 1; /* 参照されるたびに時刻を加算 */
        return self.mtime
    }
    
    fn mtimecmp_write(&mut self, addr: usize, val: u64) {
        /* ゲストタイマ割込みをフラッシュ */
        Hext::flush_vsmode_interrupt(Interrupt::bit(
            Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
        ));
        
        let current = u64::from_be(read_raw::<u64>(0x0200_bff8)) + 1000;
        sbi::sbi_set_timer((current) as u64);
        //let current = val;
        //sbi::sbi_set_timer(val);

        self.mtimecmp = val;
        self.mtime = (current) as u64;
    }

    fn mtimecmp_read(&mut self, addr: usize) -> u64 {
        return self.mtimecmp
    }
}

impl VirtualDeviceT for VClint {
    fn write(&mut self, addr: usize, val: usize) {
        /* [todo fix] レジスタ取得を関数にまとめたい */
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
