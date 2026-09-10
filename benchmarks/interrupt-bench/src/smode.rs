use core::panic::PanicInfo;
use core::arch::asm;

use print::*;

use crate::{print, println, sbi::sbi_set_timer};


const SAMPLE_COUNT: usize = 1000000;

static mut RESULT_LATENCY: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];

const TIMER_FREQ: u64 = 1000000; // 1MHz
const CYCLE_FREQ: u64 = 1400000000; // 1400MHz
const TIME_SLICE: u64 = 10; // 10us
const CYCLE_SLICE: u64 = TIME_SLICE * CYCLE_FREQ / TIMER_FREQ;

static mut TOTAL_INTERRUPTS: usize = 0;
static mut NEXT_CYCLE: u64 = 0;
static mut NEXT_TIME: u64 = 0;

static mut START_TIME: u64 = 0; // 大まかな時間計測用



/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic!: {}", _info);
    loop {}
}

/// Main program function
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn main(cpu_id: usize) {
    println!("Initializing...");
    // Initialize BSS
    init_bss();
    
    unsafe { TOTAL_INTERRUPTS = 0 };

    println!("Timer interrupt latency test");
    println!("CPU ID: {}", cpu_id);
    println!("Cycle frequency: {} Hz", CYCLE_FREQ);
    println!("Timer frequency: {} Hz", TIMER_FREQ);

    unsafe {
        let start_time: u64;
        asm!("rdtime {0}", out(reg) start_time);
        START_TIME = start_time;
        
        set_timer();
    }

}

#[no_mangle]
pub extern "C" fn trap_handler() {
    unsafe {
        let mut cycle: u64;
        asm!("rdcycle {0}", out(reg) cycle);

        let latency = cycle;
        RESULT_LATENCY[TOTAL_INTERRUPTS] = latency;
        TOTAL_INTERRUPTS += 1;
        if TOTAL_INTERRUPTS >= SAMPLE_COUNT {
            totaling_bench();
            _loop();
        }
    }
    set_timer();
}

#[inline]
fn set_timer() {
    unsafe {
        let mut current: u64;
        asm!("rdtime {0}", out(reg) current);
        let next_time = current + TIME_SLICE;
        sbi_set_timer(next_time);
        NEXT_TIME = next_time;

        asm!("
            csrw sie, {0}
            csrsi sstatus, 0x2
        ", in(reg) 1 << 5, options(nostack));
        loop {
            asm!("wfi",options(nostack));
        }
    }
}

fn totaling_bench() {
    // Print results
    let mut end_time: u64 = 0;
    unsafe { asm!("rdtime {0}", out(reg) end_time) };
    let elapsed_time = end_time as f64 - unsafe { START_TIME as f64 };
    
    let mut total_latency: u64 = 0;
    let mut max_latency: u64 = 0;
    let mut min_latency: u64 = 0xFFFFFFFFFFFFFFFF;

    for i in 0..SAMPLE_COUNT {
        let latency = unsafe { RESULT_LATENCY[i] };
        total_latency += latency;
        if latency > max_latency {
            max_latency = latency;
        }
        if latency < min_latency {
            min_latency = latency;
        }
    }

    let avg_latency_s = total_latency as f64 / (SAMPLE_COUNT as f64) / (CYCLE_FREQ as f64); // in s
    let max_latency_s = max_latency as f64 / (CYCLE_FREQ as f64); // in s
    let min_latency_s = min_latency as f64 / (CYCLE_FREQ as f64); // in s

    println!("Elapsed time      : {} ms", (elapsed_time / TIMER_FREQ as f64) * 1000.0);
    println!("Total interrupts  : {} / {}", unsafe { TOTAL_INTERRUPTS }, SAMPLE_COUNT);
    println!("Avg latency       : {} us", avg_latency_s * 1000000.0);
    println!("Max latency       : {} us", max_latency_s * 1000000.0);
    println!("Min latency       : {} us", min_latency_s * 1000000.0);

    // for i in 0..10 {
    //     println!("Latency[{}]       : {} us", i, unsafe { RESULT_LATENCY[i] } as f64 / (CYCLE_FREQ as f64) * 1000000.0);
    // }

    let start_ptr = unsafe { RESULT_LATENCY.as_ptr() as usize };
    let end_ptr = unsafe { RESULT_LATENCY.as_ptr().add(SAMPLE_COUNT) as usize};
    println!("Result address    : {:#x} - {:#x}", start_ptr, end_ptr);
}


/// Entry point called at violet startup
#[cfg(target_arch = "riscv64")]
#[link_section = ".init"]
#[export_name = "_start"]
#[naked]
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        asm! ("
        .option norvc
        .option norelax
        .align 8
                la      t0, _entry_trap
                csrw    stvec, t0
                la      sp, __KERNEL_SP_BOTTOM
                j       main
        ",
        options(noreturn)
        );
    }
}

/// Entry point called at trap
#[cfg(target_arch = "riscv64")]
#[export_name = "_entry_trap"]
#[naked]
pub extern "C" fn _entry_trap() {
    unsafe {
        asm! ("
        .align 8
                // Disable interrupts
                csrci   sstatus, 0x2
                csrwi   sie, 0
                la      sp, __KERNEL_SP_BOTTOM
                j       trap_handler
        ",
        options(noreturn)
        );
    }
}

/// Entry point called at trap
#[cfg(target_arch = "riscv64")]
#[export_name = "_loop"]
#[naked]
pub extern "C" fn _loop() {
    unsafe {
        asm! ("
        .align 8
                wfi
                j _loop
        ",
        options(noreturn)
        );
    }
}

fn init_bss() {
    // init bss, sbss
    extern "C" {
        static mut __BSS_START: u8;
        static mut __BSS_END: u8;
    }
    unsafe {
        core::ptr::write_bytes(&mut __BSS_START, 0, &__BSS_END as *const u8 as usize - &__BSS_START as *const u8 as usize);
    }
}