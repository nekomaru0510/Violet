//! ゲスト用仮想メモリの管理

extern crate alloc;
use alloc::vec::Vec;

pub struct VirtualMemoryMap {
    map: Vec<VirtualMemoryArea>,
}

impl VirtualMemoryMap {
    pub const fn new() -> Self {
        VirtualMemoryMap { map: Vec::new() }
    }

    pub fn register(&mut self, vaddr: usize, paddr: usize, size: usize) {
        /* [todo fix] メモリマップが被らないか確認 */
        self.map.push(VirtualMemoryArea::new(vaddr, paddr, size));
        //self.sort();
    }

    /* [todo fix] 実装する */
    /*
    pub fn unregister() {

    }*/

    fn sort(&mut self) {
        self.map.sort_by(|a, b| b.vaddr.cmp(&a.vaddr))
    }

    pub fn get(&self, addr: usize) -> Option<&VirtualMemoryArea> {
        self.find(addr)
    }

    pub fn get_mut(&mut self, addr: usize) -> Option<&mut VirtualMemoryArea> {
        self.find_mut(addr)
    }

    pub fn find(&self, addr: usize) -> Option<&VirtualMemoryArea> {
        self.map
            .iter()
            .rfind(|e| e.vaddr <= addr && addr < e.vaddr + e.size)
    }

    pub fn find_mut(&mut self, addr: usize) -> Option<&mut VirtualMemoryArea> {
        self.map
            .iter_mut()
            .rfind(|e| e.vaddr <= addr && addr < e.vaddr + e.size)
    }
}

pub struct VirtualMemoryArea {
    pub vaddr: usize,
    pub paddr: usize,
    pub size: usize,
}

impl VirtualMemoryArea {
    pub fn new(vaddr: usize, paddr: usize, size: usize) -> Self {
        VirtualMemoryArea { vaddr, paddr, size }
    }

    pub fn get_paddr(&self, vaddr: usize) -> Option<usize> {
        /* メモリマップ外 */
        if vaddr < self.vaddr || self.vaddr + self.size <= vaddr {
            return None;
        }
        Some(self.paddr + (vaddr - self.vaddr))
    }
}

#[test_case]
fn test_register() -> Result<(), &'static str> {
    let mut vmem = VirtualMemoryMap::new();
    let guest_paddr = 0x9000_0000;
    let real_paddr = 0x8020_0000;
    let size = 0x1000_0000;

    vmem.register(guest_paddr, real_paddr, size);
    let result = match vmem.get(0x9000_0000) {
        None => Err("Fail to get virtual memory"),
        Some(m) => {
            if m.paddr == 0x8020_0000 {
                Ok(())
            } else {
                Err("Fail to get real paddr")
            }
        }
    };

    if result != Ok(()) {
        return result;
    }

    vmem.register(guest_paddr, real_paddr, size);
    match vmem.get(0xa000_0000) {
        None => Ok(()),
        Some(m) => Err("Invalid virtual memory"),
    }
}

#[test_case]
fn test_paddr_get() -> Result<(), &'static str> {
    let mut vmem = VirtualMemoryMap::new();
    let guest_paddr = 0x9000_0000;
    let real_paddr = 0x8020_0000;
    let size = 0x1000_0000;

    vmem.register(guest_paddr, real_paddr, size);
    let result = match vmem.get(guest_paddr) {
        None => Err("Fail to get virtual memory"),
        Some(m) => {
            let paddr = m.get_paddr(guest_paddr + 0x1234);
            match paddr {
                None => Err("Fail to get real paddr 1"),
                Some(p) => {
                    if p == real_paddr + 0x1234 {
                        Ok(())
                    } else {
                        Err("Fail to get real paddr 2")
                    }
                }
            }
        }
    };

    if result != Ok(()) {
        return result;
    }

    match vmem.get(0x9000_0000) {
        None => Err("Fail to get virtual memory"),
        Some(m) => {
            if m.get_paddr(0x9000_0000 - 0x1234) == None {
                Ok(())
            } else {
                Err("Fail to get real paddr 3")
            }
        }
    }
}
