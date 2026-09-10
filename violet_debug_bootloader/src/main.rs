#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use core::arch::asm;

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::ptr;

static UART0_ADDR: usize = 0x5090_0000;
static VIOLET_ADDR: usize = 0x3_8020_0000;

pub fn write_byte(c: u8) {
    let uart0 = UART0_ADDR as *mut u8;
    // Wait for UART to become ready
    // while unsafe { ptr::read_volatile(uart0) } & 0x20 == 0 {}
    
    unsafe { asm!("sb {0}, 0({1})", in(reg) c, in(reg) uart0)};
}

fn write_string(s: &str) {
    for c in s.bytes() {
        write_byte(c);
    }
}

/// Main program function
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn main(cpu_id: usize) {
    write_string("Violet debug bootloader\r\n");

    // Init VIOLET_ADDR
    unsafe {
        ptr::write_volatile(VIOLET_ADDR as *mut u64, 0);
    }
    // Wait for debugger write the VIOLET_ADDR
    while unsafe { ptr::read_volatile(VIOLET_ADDR as *const u64) } == 0 {}

    let addr = VIOLET_ADDR;

    unsafe { 
        asm!("
            // set cpuid
            mv a0, {0}
            // set addr
            mv t0, {1}
            jalr t0
        ", in(reg) cpu_id, in(reg) addr);
    }

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
                /* a0 ... hartid */
                li      t0, 1        
                li      t1, 14
                sll     t0, t0, t1
                // [todo fix] mul instruction is not wanted, 
                // but if only shift operation is used, 
                // sp will be broken by optimization
                mul     t0, t0, a0          
                la      sp, __KERNEL_SP_BOTTOM
                add     sp, sp, t0

                j       main
        ",
        options(noreturn)
        );
    }
}