//! Virtual Registers

pub mod vmhartid;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub trait VirtualRegisterT {
    fn write(&mut self, val: u64);
    fn read(&mut self) -> u64;
}

pub struct VirtualRegister {
    id: usize,
    vreg: Box<dyn VirtualRegisterT>,
}

impl VirtualRegister {
    pub fn new<T: VirtualRegisterT + 'static>(id: usize, vreg: T) -> Self {
        VirtualRegister {
            id,
            vreg: Box::new(vreg),
        }
    }

    pub fn get(&self) -> &dyn VirtualRegisterT {
        self.vreg.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut dyn VirtualRegisterT {
        self.vreg.as_mut()
    }
}

pub struct VirtualRegisterMap {
    map: Vec<VirtualRegister>,
}

impl VirtualRegisterMap {
    pub fn new() -> Self {
        VirtualRegisterMap { map: Vec::new() }
    }

    pub fn register<T: VirtualRegisterT + 'static>(&mut self, id: usize, vreg: T) {
        self.map.push(VirtualRegister::new(id, vreg));
    }

    pub fn unregister<T: VirtualRegisterT + 'static>(&mut self, id: usize, vreg: T) {
        // [todo fix] implement unregister
    }

    pub fn get(&self, id: usize) -> Option<&dyn VirtualRegisterT> {
        match self.find(id) {
            None => None,
            Some(r) => Some(r.get()),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut dyn VirtualRegisterT> {
        match self.find_mut(id) {
            None => None,
            Some(r) => Some(r.get_mut()),
        }
    }

    pub fn find(&self, id: usize) -> Option<&VirtualRegister> {
        self.map.iter().find(|e| e.id == id)
    }

    pub fn find_mut(&mut self, id: usize) -> Option<&mut VirtualRegister> {
        self.map.iter_mut().find(|e| e.id == id)
    }
}

#[cfg(test)]
use vmhartid::Vreg;

#[test_case]
fn test_vreg() -> Result<(), &'static str> {
    let mut map = VirtualRegisterMap::new();
    let vreg = Vreg::new(0);

    map.register(1, vreg);

    let result = match map.find(1) {
        None => Err("Fail to find vreg"),
        Some(x) => Ok(()),
    };

    if result != Ok(()) {
        return result;
    }

    match map.find(0) {
        None => Ok(()),
        Some(x) => Err("Find Invalid vreg"),
    }
}
