//! Test Runner
extern crate core;
use crate::{print, println};
use core::intrinsics::transmute;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn() -> Result<(), &'static str>]) {
    let mut success = 0;
    println!("Running {} tests", tests.len());
    for test in tests {
        match test() {
            Ok(()) => {
                println!("[ok]");
                success += 1;
            }
            Err(e) => {
                let addr: usize = unsafe { transmute(test) };
                println!("[ng] Function:0x{:x} ... {}", addr, e);
            }
        }
    }
    println!("Result: {}/{} ", success, tests.len());
}

#[test_case]
fn test_test() -> Result<(), &'static str> {
    if 1 == 1 {
        Ok(())
    } else {
        Err("error")
    }
}
