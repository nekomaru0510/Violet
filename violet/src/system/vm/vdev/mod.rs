//! 仮想デバイス

pub mod vplic;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::{read_volatile, write_volatile};

pub trait VirtualDevice {
    fn write32(&mut self, addr: usize, val: u32); // [todo fix] ジェネリクスを使う
    fn read32(&mut self, addr: usize) -> u32;
    //fn write64(&mut self, addr: usize, val: u64);
    //fn read64(&mut self, addr: usize) -> u64;
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

    //pub fn find(&self, addr: usize) -> Option<&'static Io> {
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
    //map: BTreeMap<(usize, usize), Box<dyn VirtualDevice>>,
    map: IoMap,
}

impl VirtualIoMap {
    pub fn new() -> Self {
        VirtualIoMap {
            //map: BTreeMap::new(),
            map: IoMap::new(),
        }
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

    pub fn get<T: VirtualDevice + 'static>(&self, addr: usize) -> Option<&Box<dyn VirtualDevice>> {
        //pub fn get<T: VirtualDevice + 'static>(&self, addr: usize) -> Option<&'static Box<dyn VirtualDevice>> {
        match self.map.find(addr) {
            None => None,
            Some(i) => Some(&i.vdev),
        }
    }

    pub fn get_mut<T: VirtualDevice + 'static>(
        &mut self,
        addr: usize,
    ) -> Option<&mut Box<dyn VirtualDevice>> {
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

/* ゼロレジスタ */
pub struct ZeroReg {
    reg: u32,
}

impl ZeroReg {
    pub const fn new() -> Self {
        ZeroReg { reg: 0 }
    }
}

impl VirtualRegister for ZeroReg {
    type Register = u32;

    fn write(&mut self, addr: usize, val: u32) {
        ()
    }

    fn read(&mut self, addr: usize) -> u32 {
        0
    }
}
