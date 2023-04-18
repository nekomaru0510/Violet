//! 仮想PLIC

use crate::driver::traits::intc::TraitIntc;
use crate::kernel::container::*;

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    priority_threshold: PriorityThresholdReg,
    claim_comp: ClaimCompReg,
    zero: ZeroReg,
    reg: [u32; 1024],
}

impl VPlic {
    pub const fn new() -> Self {
        VPlic {
            priority_threshold: PriorityThresholdReg::new(),
            claim_comp: ClaimCompReg::new(),
            zero: ZeroReg::new(),
            reg: [0 as u32; 1024],
        }
    }

    pub fn write32(&mut self, addr: usize, val: u32) {
        /* [todo fix] レジスタ取得を関数にまとめたい */
        match addr {
            0x1000 => self.priority_threshold.write(val),
            0x1004 => self.claim_comp.write(val),
            _ => self.zero.write(val),
        };
    }

    pub fn read32(&mut self, addr: usize) -> u32 {
        match addr {
            0x1000 => self.priority_threshold.read(),
            0x1004 => self.claim_comp.read(),
            _ => self.zero.read(),
        }
    }
}

pub struct PriorityThresholdReg {
    reg: u32,
}

impl PriorityThresholdReg {
    pub const fn new() -> Self {
        PriorityThresholdReg { reg: 0 }
    }
}

impl VirtualRegister for PriorityThresholdReg {
    type Register = u32;

    fn write(&mut self, val: u32) {
        let con = current_mut_container();
        self.reg = val & 0x7;
        match &mut con.unwrap().intc {
            None => (),
            Some(i) => i.set_priority_threshold(self.reg),
        }
    }

    fn read(&mut self) -> u32 {
        self.reg
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

    fn read(&mut self) -> u32 {
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

    fn read(&mut self) -> u32 {
        0
    }
}
