use core::panic::PanicInfo;
use core::arch::asm;

use crate::println;

const TIMER_FREQ: u64 = 1000000; // 1MHz
const CYCLE_FREQ: u64 = 1400000000; // 1400MHz



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
    use crate::bench;
    // Initialize BSS
    init_bss();
    
    println!("Violet M-mode Emulator Benchmark");
    println!("CPU ID: {}", cpu_id);
    println!("Cycle frequency: {} Hz", CYCLE_FREQ);
    println!("Timer frequency: {} Hz", TIMER_FREQ);
    println!("");

    println!("M-mode CSR read/write benchmark");
    bench::csr::bench_all();
    println!("");
    println!("CLINT read/write benchmark");
    bench::clint::bench_all();

    loop {}
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
                // If mhartid is 0, jump to main
                // Otherwise, jump to _loop
                csrr     a0, mhartid
                bne      a0, zero, _loop
                la      sp, __KERNEL_SP_BOTTOM
                j       main
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