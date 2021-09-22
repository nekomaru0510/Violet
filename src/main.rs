#![no_main]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]
#![no_std]

mod driver;
mod library;
mod resource;
mod service;

mod container;
use container::TraitContainer;

pub mod minimum_allocator;

/* 使用するコンテナを登録 */
mod sample_container;
use sample_container::SampleContainer;
use service::vshell::VShell;

// test
//use crate::kernel::resource::io::fesyscall::FeSyscall;
use lazy_static::lazy_static;
/*
lazy_static! {
    static ref SAMPLE_CONTAINER: SampleContainer = SampleContainer::new();
}*/

//static mut SAMPLE_CONTAINER: SampleContainer = SampleContainer::new();
//static mut SAMPLE_CONTAINER: SampleContainer;
#[allow(improper_ctypes)]
extern "C" {
    static mut SAMPLE_CONTAINER: SampleContainer;
}

#[no_mangle]
pub extern "C" fn boot_init() -> ! {
    /*
    unsafe{
        let mut fe = FeSyscall::new();
        fe.sys_write(1, &("hello".as_bytes())[0] as *const u8 , 5);

        let mut buf: [u8; 32] = [0; 32];

        fe.sys_write(1, &("1\n".as_bytes())[0] as *const u8 , 2);

        while buf[0] == 0 as u8 {
            fe.sys_read(0, &buf[0] as *const u8 , 5);
        }

        fe.sys_write(1, &(buf[0] as u8) as *const u8 , 1);

        fe.sys_exit();
    }       */
    //let mut kernel = Kernel::new();
    //kernel.run();
    
    unsafe {
        SAMPLE_CONTAINER = SampleContainer::new();
        SAMPLE_CONTAINER.run();
    }
    //let mut con = SampleContainer::new();
    //con.run();

    loop {}
}

#[no_mangle]
pub extern "C" fn interrupt_handler(cont: &mut Context) {
    unsafe {
        /* 各種コンテナへ割込みの振分け */
        SAMPLE_CONTAINER.interrupt(cont);
    }
}

/* 割込み元のコンテキストを示す */
#[derive(Clone, Copy)]
pub struct Context {
    cpuid: u8,
    regs : [usize; 32],
    sp : *mut usize,
    regsize: u32,
}

impl Context {
    pub fn new() -> Self {
        Context{cpuid:0, regs: [0; 32], sp:0 as *mut usize, regsize:0, }
    }
}

/* 無いとコンパイルエラー(言語仕様) */
use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* 無いとコンパイルエラー */
#[no_mangle]
pub extern "C" fn abort(_info: &PanicInfo) -> ! {
    loop {}
}
