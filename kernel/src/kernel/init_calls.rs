//! init_calls関連処理


#[allow(improper_ctypes)]
extern "C" {
    static mut INIT_CALLS_HEAD: [Option<fn()>; 8];
}

pub fn do_init_calls() {
    unsafe {
        match INIT_CALLS_HEAD[0] {
            Some(func) => {
                func();
            },
            None => {
    
            }
        }    
    }    
}
