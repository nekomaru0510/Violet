//! 仮想PLIC

use crate::driver::traits::intc::TraitIntc;
use crate::kernel::container::*;
use super::VirtualDevice;
use super::VirtualRegister;
use super::ZeroReg;
use super::{read_raw, write_raw};

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    priority_threshold: PriorityThresholdReg,
    claim_comp: ClaimCompReg,
    zero: ZeroReg,
    //reg: [u32; 1024],
}

/*
const START_HART0_M_INT_ENABLE0: usize = 0x2000;
const START_HART0_M_INT_ENABLE1: usize = 0x2004;
const START_HART1_M_INT_ENABLE0: usize = 0x2080;
const START_HART1_M_INT_ENABLE1: usize = 0x2084;
const START_HART1_S_INT_ENABLE0: usize = 0x2100;
const START_HART1_S_INT_ENABLE1: usize = 0x2104;
const START_HART2_M_INT_ENABLE0: usize = 0x2180;
const START_HART2_M_INT_ENABLE1: usize = 0x2184;
const START_HART2_S_INT_ENABLE0: usize = 0x2200;
const START_HART2_S_INT_ENABLE1: usize = 0x2204;
const START_HART3_M_INT_ENABLE0: usize = 0x2280;
const START_HART3_M_INT_ENABLE1: usize = 0x2284;
const START_HART3_S_INT_ENABLE0: usize = 0x2300;
const START_HART3_S_INT_ENABLE1: usize = 0x2304;

const HART0_PRIO_THRESHOLD: usize = 0x20_1000;
const HART0_CLAIM_COMPLETE: usize = 0x20_1004;
*/

impl VPlic {
    pub const fn new() -> Self {
        VPlic {
            priority_threshold: PriorityThresholdReg::new(),
            claim_comp: ClaimCompReg::new(),
            zero: ZeroReg::new(),
            //reg: [0 as u32; 1024],
        }
    }
}

impl VirtualDevice for VPlic {
    fn write32(&mut self, addr: usize, val: u32) {
        /* [todo fix] レジスタ取得を関数にまとめたい */
        match addr {
            0x1000 => self.priority_threshold.write(val),
            0x1004 => self.claim_comp.write(val),
            _ => write_raw(addr, val),//self.zero.write(val),
        };
    }

    fn read32(&mut self, addr: usize) -> u32 {
        match addr {
            0x1000 => self.priority_threshold.read(),
            0x1004 => self.claim_comp.read(),
            _ => read_raw(addr), //self.zero.read(),
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

