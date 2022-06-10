//! 仮想PLIC

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    claim_comp: ClaimCompReg,
    zero: ZeroReg,
    reg: [u32; 1024],
}

impl VPlic {
    pub const fn new() -> Self {
        VPlic { 
            claim_comp: ClaimCompReg::new(), 
            zero: ZeroReg::new(),
            reg: [0 as u32; 1024], 
        }
    }

    pub fn write32(&mut self, addr: usize, val: u32) {
        /* [todo fix] 関数にまとめたい */
        let mut vreg = match addr {
            0x1004 => self.claim_comp.write(val),
            _ => self.zero.write(val)
        };
        //(*vreg).write(val);
    }

    pub fn read32(&mut self, addr: usize) -> u32{
        match addr {
            0x1004 => self.claim_comp.read(),
            _ => self.zero.read()
        }
        //(*vreg).read()
    }
}

pub struct ClaimCompReg {
    reg: u32,
}

impl ClaimCompReg {
    pub const fn new() -> Self {
        ClaimCompReg { reg: 0 }
    }
}

impl VirtualRegister for ClaimCompReg {
    type Register = u32;

    fn write(&mut self, val: u32) {
        self.reg = val;
    }

    fn read(&mut self) -> u32{
        let result = self.reg;
        self.reg = 0;
        result
    }  
}


trait VirtualRegister {
    type Register;

    fn write(&mut self, val: Self::Register);
    fn read(&mut self) -> Self::Register; /* 読み出し時にレジスタ値を変更するものも存在するため、mutable */
}


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

    fn write(&mut self, val: u32) {
        ()
    }

    fn read(&mut self) -> u32{
        0
    }  
}
