//! 仮想PLIC

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    reg: [u32; 1024],
}

impl VPlic {
    pub const fn new() -> Self {
        VPlic { reg: [0 as u32; 1024], }
    }

    pub fn write32(&mut self, addr: usize, val: u32) {
        self.reg[addr & 0x3ff] = val;
    }

    pub fn read32(&self, addr: usize) -> u32{
        self.reg[addr & 0x3ff]
    }

}
