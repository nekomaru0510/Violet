//! Violetアプリケーションのサンプル
//! Linuxカーネルを動作させる

#![no_main]
#![no_std]

extern crate violet;

use violet::{print, println};

#[link_section = ".init_calls"]
#[no_mangle]
pub static mut INIT_CALLS: Option<fn()> = Some(init_sample);

pub fn init_sample() {
    println!("sample application init !!");
}
