//! 仮想デバイス

pub mod vplic;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::{read_volatile, write_volatile};

pub trait VirtualDevice {
    fn write(&mut self, addr: usize, val: usize);
    fn read(&mut self, addr: usize) -> usize;
    fn interrupt(&mut self, intid: usize);
}

pub trait VirtualRegister {
    type Register;

    fn write(&mut self, addr: usize, val: Self::Register);
    fn read(&mut self, addr: usize) -> Self::Register; /* 読み出し時にレジスタ値を変更するものも存在するため、mutable */
}

struct IoMap {
    map: Vec<Io>,
}

impl IoMap {
    pub fn new() -> Self {
        IoMap { map: Vec::new() }
    }

    pub fn register(&mut self, dev: Io) {
        self.map.push(dev);
        self.sort();
    }
    /*
    pub fn unregister(&mut self, base: usize) {

    }
    */
    fn sort(&mut self) {
        self.map.sort_by(|a, b| b.base.cmp(&a.base))
    }

    pub fn find(&self, addr: usize) -> Option<&Io> {
        self.map
            .iter()
            .find(|e| e.base <= addr && addr < e.base + e.size)
    }

    pub fn find_mut(&mut self, addr: usize) -> Option<&mut Io> {
        self.map
            .iter_mut()
            .find(|e| e.base <= addr && addr < e.base + e.size)
    }
}

struct Io {
    base: usize,
    size: usize,
    vdev: Box<dyn VirtualDevice>,
}

impl Io {
    pub fn new<T: VirtualDevice + 'static>(base: usize, size: usize, vdev: T) -> Self {
        Io {
            base,
            size,
            vdev: Box::new(vdev),
        }
    }
}

pub struct VirtualIoMap {
    map: IoMap,
}

impl VirtualIoMap {
    pub fn new() -> Self {
        VirtualIoMap { map: IoMap::new() }
    }

    pub fn register<T: VirtualDevice + 'static>(&mut self, base: usize, size: usize, vdev: T) {
        self.map.register(Io::new(base, size, vdev));
    }

    pub fn unregister<T: VirtualDevice + 'static>(
        &mut self,
        base_addr: usize,
        size: usize,
        vdev: T,
    ) {
        // [todo fix] 実装する
    }

    pub fn get(&self, addr: usize) -> Option<&Box<dyn VirtualDevice>> {
        match self.map.find(addr) {
            None => None,
            Some(i) => Some(&i.vdev),
        }
    }

    pub fn get_mut(&mut self, addr: usize) -> Option<&mut Box<dyn VirtualDevice>> {
        match self.map.find_mut(addr) {
            None => None,
            Some(i) => Some(&mut i.vdev),
        }
    }
}

pub fn read_raw<T>(addr: usize) -> T {
    unsafe { read_volatile(addr as *const T) }
}

pub fn write_raw<T>(addr: usize, val: T) {
    unsafe {
        write_volatile(addr as *mut T, val);
    }
}

#[cfg(test)]
use crate::library::vm::vdev::vplic::VPlic;

#[test_case]
fn test_get() -> Result<(), &'static str> {
    let mut map = VirtualIoMap::new();
    let vplic = VPlic::new();
    map.register(0x0c00_0000, 0x0400_0000, vplic);

    let mut result = match map.get(0x0c00_0000) {
        None => Err("can't get virtual device"),
        Some(d) => Ok(()),
    };

    if result != Ok(()) {
        return result;
    };

    result = match map.get(0x0bff_ffff) {
        None => Ok(()),
        Some(d) => Err("invalid virtual device"),
    };

    if result != Ok(()) {
        return result;
    };

    result = match map.get(0x0fff_ffff) {
        None => Err("can't get virtual device"),
        Some(d) => Ok(()),
    };

    if result != Ok(()) {
        return result;
    };

    result = match map.get(0x1000_0000) {
        None => Ok(()),
        Some(d) => Err("invalid virtual device"),
    };

    result
}
