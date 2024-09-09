//! Error handling for the Violet project.

use core::fmt;
use core::error::Error;
use crate::alloc::string::ToString;
use alloc::string::String;

#[derive(Debug)]
struct UndefinedError {
    details: String,
}

impl UndefinedError {
    fn new(msg: &str) -> UndefinedError {
        UndefinedError { details: msg.to_string() }
    }
}

impl fmt::Display for UndefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UndefinedError: {}", self.details)
    }
}

impl Error for UndefinedError {}

// Runtime error in Violet
#[derive(Debug)]
enum RuntimeError {
    NotImpl,
    Undef(UndefinedError),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::NotImpl => write!(f, "Not implemented"),
            RuntimeError::Undef(err) => write!(f, "{}", err),
        }
    }
}

impl Error for RuntimeError {}

#[cfg(test)]
fn return_error() -> Result<(), RuntimeError> {
    Err(RuntimeError::Undef(UndefinedError::new("Test error")))
    //Err(RuntimeError::NotImpl)
}

#[test_case]
pub fn test_error() -> Result<(), &'static str> {
    match return_error() {
        Ok(_) => Err("Error not returned"),
        Err(e) => {Ok(())},
    }
}