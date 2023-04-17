//! init_calls関連処理

#[link_section = ".init_calls.1.end"]
#[no_mangle]
static __INIT_CALLS_1_END: Option<fn()> = None;

#[link_section = ".init_calls.2.end"]
#[no_mangle]
static __INIT_CALLS_2_END: Option<fn()> = None;

const MAX_INIT_CALLS_1: usize = 128;
const MAX_INIT_CALLS_2: usize = 128;

#[allow(improper_ctypes)]
extern "C" {
    static mut __INIT_CALLS_1_START: [Option<fn()>; MAX_INIT_CALLS_1];
    static mut __INIT_CALLS_2_START: [Option<fn()>; MAX_INIT_CALLS_2];
}

pub fn do_init_calls() {
    do_driver_calls();
    do_app_calls();
}

pub fn do_driver_calls() {
    unsafe {
        for i in (0 .. MAX_INIT_CALLS_1) {
            match __INIT_CALLS_1_START[i] {
                Some(func) => {
                    func();
                }
                None => {
                    break;
                }
            }
        }
    }
}

pub fn do_app_calls() {
    unsafe {
        for i in (0 .. MAX_INIT_CALLS_2) {
            match __INIT_CALLS_2_START[i] {
                Some(func) => {
                    func();
                }
                None => {
                    break;
                }
            }
        }
    }
}

#[macro_export]
macro_rules! driver_init {
    ($func:path) => {
        #[link_section = ".init_calls.1.start"]
        #[used]
        pub static __DRIVER_INIT_FUNC: Option<fn()> = Some($func);
    };
}

#[macro_export]
macro_rules! app_init {
    ($func:path) => {
        #[link_section = ".init_calls.2.start"]
        #[used]
        pub static __APP_INIT_FUNC: Option<fn()> = Some($func);
    };
}

