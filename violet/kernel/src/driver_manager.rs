//! Driver Manager
use core ::mem::transmute;
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[link_section = ".init_calls"]
static mut init_call: fn()  = init_calls_test;

const MAX_INIT_CALLS: usize = 10;
extern "C" {
    static mut init_calls_top: [usize; MAX_INIT_CALLS];
    //static mut calls: Box<Vec<T>>;
}

#[macro_export]
macro_rules! module_init {
    ($i:expr) => (
        #[link_section = ".init_calls"]
        static mut INIT_CALL: fn()->usize = $i;
    );
    ( $i:expr, $t:ty ) => (
        #[link_section = ".init_calls"]
        static mut INIT_CALL: fn()->$t = $i;
    );
}

/*
fn return_driver<T>(drv: T) -> usize {
    unsafe {
        let ret: usize = transmute(drv);
        ret
    }
}
*/

pub struct DriverManager;

impl DriverManager {

    pub fn new() -> Self {
        DriverManager{}
    }

    pub fn call_initializer(&self) {
        unsafe {
            let calls: [fn(); MAX_INIT_CALLS] = transmute(init_calls_top);
            let dummy: fn() = transmute(0 as usize);
            for i in 0..MAX_INIT_CALLS {
                if calls[i] == dummy {
                    break;
                }
                let hoge = calls[i]();
            }
        }
   }
}

fn init_calls_test() {
    //println!("init calls ok!");
}
