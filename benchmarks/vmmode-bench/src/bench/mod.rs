pub mod csr;
pub mod clint;

use core::arch::asm;

use crate::println;

#[inline(always)]
pub fn poison_cache() {
    // Fill a large array with data to ensure it is not in the cache.
    static mut DUMMY: [u64; 4096] = [0; 4096];
    unsafe {
        for i in 0..DUMMY.len() {
            core::ptr::write_volatile(&mut DUMMY[i], i as u64);
        }
        // Flush the instruction cache to ensure the next instructions are not cached.
        asm!(
            "fence.i",
            options(nostack)
        );
    }
}

pub fn print_stats(label: &str, op: &str, data: &[u64]) {
    // let mut total = 0u64;
    // let mut max = 0u64;
    // let mut min = u64::MAX;
    // for &cycle in data {
    //     total += cycle;
    //     if cycle > max { max = cycle; }
    //     if cycle < min { min = cycle; }
    // }
    // let avg = total as f64 / data.len() as f64;
    // println!("{} {} cycle: avg: {:.2}, min: {}, max: {}", label, op, avg, min, max);
    let start = data.as_ptr() as usize;
    // let start = data.as_ptr() as usize + 0x4000_0000; // When bench on VioletVM
    let end = start + data.len() * 8 - 1;

    println!("dump binary memory {}_{}.bin {:#x} {:#x}", label, op, start, end + 1);
}
