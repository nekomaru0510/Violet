//! Test関連
use crate::{print, println};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn test_test() {
    assert_eq!(1, 1);
    println!("[ok]");
}
