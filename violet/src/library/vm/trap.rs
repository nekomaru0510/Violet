//! Virtual Machine Trap Module

use alloc::vec::Vec;
use crate::environment::Arch;
use crate::arch::traits::TraitArch;

pub struct TrapMap {
    map: Vec<Trap>,
}

impl TrapMap {
    pub const fn new() -> Self {
        TrapMap { map: Vec::new() }
    }

    pub fn register_trap(&mut self, id: usize, func: fn(regs: *mut usize)) -> Result<(), ()> {
        self.map.push(Trap::new(id, func));
        match Arch::enable_vector(id) {
            Err(_) => return Err(()),
            _ => (),
        }
        Arch::register_vector(id, func)
    }

    pub fn register_traps(&mut self, traps: &[(usize, fn(regs: *mut usize))]) -> Result<(), ()> {
        for &(id, func) in traps {
            self.map.push(Trap::new(id, func));
            match Arch::enable_vector(id) {
                Err(_) => return Err(()),
                _ => (),
            }
            Arch::register_vector(id, func)?;
        }
        Ok(())
    }

    pub fn trap_handler(id: usize, regs: *mut usize) -> Result<(), ()> {
        Arch::call_vector(id, regs)
    }
}

pub struct Trap {
    id: usize,
    func: fn(regs: *mut usize),
}

impl Trap {
    pub fn new(id: usize, func: fn(regs: *mut usize)) -> Self {
        Trap { id, func }
    }
}