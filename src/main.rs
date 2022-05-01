#![no_main]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
/* テスト用 */
#![feature(custom_test_frameworks)]
#![test_runner(crate::container::hypervisor_container::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std]

/* Violetの中核機能(なるべく小さくしたい) */
mod kernel;

/* Violetコンテナ(一般的なコンテナとは違う)。ここにカーネルを構築する */
//mod container;

/* コンテナの構成要素 */
mod driver;
mod library;
//mod resource;
//mod service;
mod environment;
mod system;

/* 使用するコンテナを登録 */
/*
use container::TraitContainer;
use container::sample_container::SampleContainer;
use container::hypervisor_container::HypervisorContainer;
*/

/*
lazy_static! {
    static ref CONTAINERS: SampleContainer = SampleContainer::new();
}*/

/* [todo fix] 可変なグローバル変数として登録。割込み処理時に必須だが、unsafeなのでなんとかしたい */
/* [todo fix] リンカスクリプトに記載する必要があり、移植性度外視なので、消したい */
/*
#[allow(improper_ctypes)]
extern "C" {
    //static mut CONTAINERS: SampleContainer;
    //static mut CONTAINERS: HypervisorContainer;
}
*/

/* 環境依存 */
use crate::environment::qemu::Qemu;
use crate::environment::qemu::init_peripherals;

pub static mut PERIPHERALS: Qemu = Qemu {
    cpu: None,
    serial: None,
    timer: None,
    intc: None,
};

//pub static mut MEMORY

/* システム依存 */
use crate::system::hypervisor::Hypervisor;

use crate::kernel::slab_allocator::init_allocater;

#[no_mangle]
pub extern "C" fn boot_init() -> ! {
    /* メモリアロケータの初期化 */
    init_allocater(0x8004_0000, 0x8006_0000);

    #[cfg(test)]
    test_main();

    init_peripherals();

    let hv = Hypervisor::new();
    hv.run();

    /*
    unsafe {
        //CONTAINERS = SampleContainer::new();
        CONTAINERS = HypervisorContainer::new();
        CONTAINERS.run();
    }*/

    loop {}
}
/*
#[no_mangle]
pub extern "C" fn interrupt_handler(cont: &mut Context) {
    unsafe {
        /* 各種コンテナへ割込みの振分け */
        //CONTAINERS.interrupt(cont);
    }
}
 */
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
