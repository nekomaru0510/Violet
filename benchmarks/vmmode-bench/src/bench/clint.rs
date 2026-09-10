use core::sync::atomic::fence;
use core::arch::asm;
use crate::println;

use super::print_stats;

const SAMPLE_COUNT: usize = 10000;
const MTIME_OFFSET: usize = 0xbff8;
const MTIMECMP_OFFSET: usize = 0x4000;

const BASE_ADDRESS: usize = 0x200_0000; 

static mut RESULT_R_MTIME: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
pub fn bench_read_mtime() {
    let mut val: u64 = 0;
    for i in 0..SAMPLE_COUNT {
        let start: u64;
        let end: u64;
        unsafe {
            // fence(core::sync::atomic::Ordering::SeqCst);
            asm!("rdcycle {0}", out(reg) start);
            val = core::ptr::read_volatile((BASE_ADDRESS + MTIME_OFFSET) as *const u64);
            asm!("rdcycle {0}", out(reg) end);
            // fence(core::sync::atomic::Ordering::SeqCst);
            RESULT_R_MTIME[i] = end - start;
        }
    }
    // println!("read mtime: {}", val);
    print_stats("mtime", "read", unsafe { core::slice::from_raw_parts(RESULT_R_MTIME.as_ptr(), SAMPLE_COUNT) });
}

static mut RESULT_R_MTIMECMP: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
pub fn bench_read_mtimecmp() {
    let mut val: u64 = 0;
    for i in 0..SAMPLE_COUNT {
        let start: u64;
        let end: u64;
        unsafe {
            // fence(core::sync::atomic::Ordering::SeqCst);
            asm!("rdcycle {0}", out(reg) start);
            val = core::ptr::read_volatile((BASE_ADDRESS + MTIMECMP_OFFSET) as *const u64);
            asm!("rdcycle {0}", out(reg) end);
            // fence(core::sync::atomic::Ordering::SeqCst);
            RESULT_R_MTIMECMP[i] = end - start;
        }
    }
    // println!("read mtimecmp: {}", val);
    print_stats("mtimcmp", "read", unsafe { core::slice::from_raw_parts(RESULT_R_MTIMECMP.as_ptr(), SAMPLE_COUNT) });
}

static mut RESULT_W_MTIMECMP: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
pub fn bench_write_mtimecmp() {
    unsafe {
        asm!("csrw mie, {0}", in(reg) 0);
    }
    
    let val: u64 = u64::MAX;
    for i in 0..SAMPLE_COUNT {
        let start: u64;
        let end: u64;
        unsafe {
            asm!("rdcycle {0}", out(reg) start);
            // fence(core::sync::atomic::Ordering::SeqCst);
            core::ptr::write_volatile((BASE_ADDRESS + MTIMECMP_OFFSET) as *mut u64, val);
            // fence(core::sync::atomic::Ordering::SeqCst);
            asm!("rdcycle {0}", out(reg) end);
            RESULT_W_MTIMECMP[i] = end - start;
        }
    }
    // println!("write mtimecmp: {}", val);
    print_stats("mtimcmp", "write", unsafe { core::slice::from_raw_parts(RESULT_W_MTIMECMP.as_ptr(), SAMPLE_COUNT) });
}

pub fn bench_all() {
    bench_read_mtime();
    bench_read_mtimecmp();
    bench_write_mtimecmp();
}