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
    get_time: fn()->u64,
}

#[derive(Debug)]
pub struct Log<T> {
    time: u64,
    value: T,
}

#[macro_export]
macro_rules! log_define {
    ($name:ident, $type:ty, $func:ident) => {
        static mut $name: LogManager<$type> = LogManager::new($func);
    };
}

#[macro_export]
macro_rules! log_record {
    ($name:ident, $value:expr) => {
        unsafe {
            $name.log($value);
        }
    };
}

#[macro_export]
macro_rules! log_get {
    ($name:ident, $idx:expr) => {
        unsafe {
            $name.get($idx)
        }
    };
}

impl<T> LogManager<T> {
    pub const fn new(func: fn()->u64) -> Self {
        LogManager {
            logs: Vec::new(),
            enable: true,
            get_time: func,
        }
    }

    pub fn log(&mut self, value: T) {
        if self.enable {
            
            self.logs.push(Log {
                time: (self.get_time)(),
                value,
            });
        }
    }

    pub fn get(&self, index: usize) -> Option<&Log<T>> {
        self.logs.get(index)
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

    pub fn get_time(&self) -> u64 {
        self.time
    }
}

#[cfg(test)]
fn get_time() -> u64 {
    0
}

#[cfg(test)]
log_define!(TEST, u64, get_time);

#[test_case]
fn test_logging() -> Result<(), &'static str> {

    log_record!(TEST, 0x1234);
    log_record!(TEST, 0x5678);

    match log_get!(TEST, 0) {
        Some(log) => {
            if *(log.get()) == 0x1234 && log.get_time() == 0 {
                Ok(())
            } else {
                Err("Fail to get log")
            }
        },
        None => Err("Fail to get log"),
    }
}
