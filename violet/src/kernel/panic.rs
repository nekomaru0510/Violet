//! Panic Function
use crate::{println, print};
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    if let Some(s) = panic.payload().downcast_ref::<&str>() {
        println!("[Violet] panic occurred: {:?}", s);
    } else {
        println!("[Violet] panic occurred");
    }

    if let Some(location) = info.location() {
        println!("[Violet] panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        ); 
    }    

    loop {}
}

#[no_mangle]
pub extern "C" fn abort(_info: &PanicInfo) -> ! {
    loop {}
}