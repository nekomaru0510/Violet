//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Panic Function
use crate::{print, println};
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[Violet] panic occurred in file '{}' at line {}",
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
