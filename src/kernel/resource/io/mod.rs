//! io module

/* extern crate core;
use core::cell::RefCell; */

pub mod serial;
use serial::Serial;

pub mod timer;
use timer::Timer;

//use crate::kernel::rwlock::RwLock;

pub struct IO {
    pub serial: Serial,
    //pub serial2: RwLock<Serial>,
    pub timer: Timer,
}

impl IO {
    pub fn new() -> Self {
        IO {
            serial: Serial::new(), 
            //serial2: RwLock::new(Serial::new()),  //test
            timer: Timer::new()
        }
    }

}