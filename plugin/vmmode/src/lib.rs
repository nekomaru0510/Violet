//! Virtual M-mode plugin
#![no_std]

extern crate violet;
use violet::{println, print};

use violet::app_init;
app_init!(vmmode_main);

fn vmmode_main() {
    println!("init vmmode plugin");
}

