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

    let mut con = SampleContainer::new();
    con.run();

    loop {}
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
