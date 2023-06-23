//! 仮想デバイス

pub mod vplic;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::{read_volatile, write_volatile};

pub trait VirtualDeviceT {
    fn write(&mut self, addr: usize, val: usize);
    fn read(&mut self, addr: usize) -> usize;
    fn interrupt(&mut self, intid: usize);
}

pub trait VirtualRegister {
    type Register;

    fn write(&mut self, addr: usize, val: Self::Register);
    fn read(&mut self, addr: usize) -> Self::Register; /* 読み出し時にレジスタ値を変更するものも存在するため、mutable */
}

pub struct VirtualDevice {
    base: usize,
    size: usize,
    vdev: Box<dyn VirtualDeviceT>,
}

impl VirtualDevice {
    pub fn new<T: VirtualDeviceT + 'static>(base: usize, size: usize, vdev: T) -> Self {
        VirtualDevice {
            base,
            size,
            vdev: Box::new(vdev),
        }
    }
}

pub struct VirtualDevMap {
    map: Vec<VirtualDevice>,
}

impl VirtualDevMap {
    pub const fn new() -> Self {
        VirtualDevMap { map: Vec::new() }
    }

    pub fn register<T: VirtualDeviceT + 'static>(&mut self, base: usize, size: usize, vdev: T) {
        self.map.push(VirtualDevice::new(base, size, vdev));
        self.sort();
    }

    pub fn unregister<T: VirtualDeviceT + 'static>(
        &mut self,
        base_addr: usize,
        size: usize,
        vdev: T,
    ) {
        // [todo fix] 実装する
    }

    fn sort(&mut self) {
        self.map.sort_by(|a, b| b.base.cmp(&a.base))
    }

    pub fn find(&self, addr: usize) -> Option<&VirtualDevice> {
        self.map
            .iter()
            .find(|e| e.base <= addr && addr < e.base + e.size)
    }

    pub fn find_mut(&mut self, addr: usize) -> Option<&mut VirtualDevice> {
        self.map
            .iter_mut()
            .find(|e| e.base <= addr && addr < e.base + e.size)
    }

    pub fn get(&self, addr: usize) -> Option<&Box<dyn VirtualDeviceT>> {
        match self.find(addr) {
            None => None,
            Some(i) => Some(&i.vdev),
        }
    }

    pub fn get_mut(&mut self, addr: usize) -> Option<&mut Box<dyn VirtualDeviceT>> {
        match self.find_mut(addr) {
            None => None,
            Some(i) => Some(&mut i.vdev),
        }
    }

    pub fn write(&mut self, addr: usize, val: usize) -> Option<()> {
        match self.get_mut(addr) {
            None => None,
            Some(d) => {
                d.write(addr, val);
                Some(())
            }
        }
    }

    pub fn read(&mut self, addr: usize) -> Option<usize> {
        match self.get_mut(addr) {
            None => None,
            Some(d) => Some(d.read(addr) as usize),
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
    let mut map = VirtualDevMap::new();
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
