//! Logging utilities

use alloc::vec::Vec;

enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct LogManager<T> {
    logs: Vec<Log<T>>,
    enable: bool,
}

#[derive(Debug)]
pub struct Log<T> {
    time: u64,
    value: T,
}

macro_rules! log_define {
    ($name:ident, $type:ty) => {
        static mut $name: LogManager<$type> = LogManager::new();
    };
}

macro_rules! log_record {
    ($name:ident, $value:expr) => {
        unsafe {
            $name.log($value);
        }
    };
}

macro_rules! log_get {
    ($name:ident, $idx:expr) => {
        unsafe {
            match $name.get($idx) {
                Some(log) => Some(*log),
                None => None,
            }
        }
    };
}

impl<T> LogManager<T> {
    pub const fn new() -> Self {
        LogManager {
            logs: Vec::new(),
            enable: true,
        }
    }

    pub fn log(&mut self, value: T) {
        if self.enable {
            self.logs.push(Log {
                time: 0,
                value,
            });
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.logs.get(index) {
            Some(log) => Some(log.get()),
            None => None,
        }
    }

}

impl<T> Log<T> {
    pub const fn new(time: u64, value: T) -> Self {
        Log {
            time,
            value,
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}

#[cfg(test)]
log_define!(TEST, u64);

#[test_case]
fn test_logging() -> Result<(), &'static str> {
    log_record!(TEST, 0x1234);
    log_record!(TEST, 0x5678);
    match log_get!(TEST, 0) {
        Some(log) => {
            if log == 0x1234 {
                Ok(())
            } else {
                Err("Fail to get log")
            }
        },
        None => Err("Fail to get log"),
    }
}
