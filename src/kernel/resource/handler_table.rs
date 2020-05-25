//! Interrupt Handler Table

extern crate core;
use core::option::Option;

pub struct HandlerTable {
    func: [Option<fn()>; 32]
}

impl HandlerTable {
    pub fn new() -> Self {
        HandlerTable{func: [None; 32], }
    }

    pub fn register(&mut self, id: u32, func: fn()) {
        self.func[id as usize] = Some(func);
    }

    pub fn call(&self, id: u32) {
        match self.func[id as usize] {
            Some(x) => (x)(),
            None => ()
        }
    }
}



