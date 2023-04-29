//! 仮想PLIC

use crate::driver::traits::intc::TraitIntc;
use crate::kernel::container::*;
use crate::environment::NUM_OF_CPUS;
use crate::driver::arch::rv64::get_cpuid; // [todo delete] //test
use super::VirtualDevice;
use super::VirtualRegister;
use super::ZeroReg;
use super::{read_raw, write_raw};

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    enable: EnableReg,
    priority_threshold: PriorityThresholdReg,
    //priority_threshold: u32,
    claim_comp: ClaimCompReg,
    //claim_comp: u32,
    zero: ZeroReg,
    interrupt: [InterruptState; 64],
}

/* 
 * 1. 割り込み発生 -> ACTIVEに
 * 2. ゲストから割り込み完了通知 -> INACTIVEに
 */
#[derive(Copy, Clone)]
enum InterruptState {
    INACTIVE,
    ACTIVE,
    //PENDING,
}

const INT_ENABLE0_CONTEXT0: usize = 0x2000;
const PRIO_THRESHOLD_CONTEXT1: usize = 0x20_1000; // S-mode Hart0
const CLAIM_COMPLETE_CONTEXT1: usize = 0x20_1004; // S-mode Hart0

const BASE_ADDRESS: usize = 0xC00_0000; /* [todo delete] */
const ADDRESS_RANGE: usize = 0x400_0000;
const MASK: usize = 0x3ff_ffff; /* [todo fix] 上記要素から算出できるように */

impl VPlic {
    pub const fn new() -> Self {
        VPlic {
            enable: EnableReg::new(),
            priority_threshold: PriorityThresholdReg::new(),
            //priority_threshold: 0,
            claim_comp: ClaimCompReg::new(),
            //claim_comp: 0,
            zero: ZeroReg::new(),
            interrupt: [InterruptState::INACTIVE; 64],
            //reg: [0 as u32; 1024],
        }
    }

    pub fn set_vcpu_config(&mut self, v2p_cpu: [usize; NUM_OF_CPUS]) {
        self.enable.set_vcpu_config(v2p_cpu);
    }

    fn claim_comp_write(&mut self, addr: usize, val: u32) {
        /*
        if (self.claim_comp.reg == val) {
            self.claim_comp.reg = 0;
        }
        else {
            self.claim_comp.reg = val;
        }*/
    }

    fn claim_comp_read(&mut self, addr: usize) -> u32 {
        let result = self.claim_comp.reg;

        let con = current_mut_container();
        self.claim_comp.reg = match &mut con.unwrap().intc {
            None => 0,
            Some(i) => i.get_pend_int(),
        };
        result
    }

    fn claim_comp_int(&mut self, intid: u32) {
        self.claim_comp.reg = intid as u32;
    }
}

impl VirtualDevice for VPlic {
    fn write32(&mut self, addr: usize, val: u32) {
        /* [todo fix] レジスタ取得を関数にまとめたい */
        match addr & MASK {
            //0x1000 => self.priority_threshold.write(addr, val),
            //PRIO_THRESHOLD_CONTEXT1 => self.priority_threshold.write(addr & MASK, val),
            //CLAIM_COMPLETE_CONTEXT1 => self.claim_comp.write(addr & MASK, val),
            CLAIM_COMPLETE_CONTEXT1 => self.claim_comp_write(addr & MASK, val),
            _ => write_raw(addr, val),
        };
    }

    fn read32(&mut self, addr: usize) -> u32 {
        match addr & MASK {
            //PRIO_THRESHOLD_CONTEXT1 => self.priority_threshold.read(addr & MASK),
            //CLAIM_COMPLETE_CONTEXT1 => self.claim_comp.read(addr & MASK),
            CLAIM_COMPLETE_CONTEXT1 => self.claim_comp_read(addr & MASK),
            _ => read_raw(addr),
        }
    }

    fn interrupt(&mut self, intid: usize) {
        self.claim_comp_int(intid as u32);
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

    fn write(&mut self, addr: usize, val: u32) {
        let con = current_mut_container();
        self.reg = val & 0x7;
        match &mut con.unwrap().intc {
            None => (),
            Some(i) => i.set_priority_threshold(self.reg),
        }
    }

    fn read(&mut self, addr: usize) -> u32 {
        self.reg
    }
}


pub struct EnableReg {
    reg: u32,
    v2p_cpu: [usize; NUM_OF_CPUS],
}

impl EnableReg {
    pub const fn new() -> Self {
        EnableReg { 
            reg: 0,
            v2p_cpu: [0; NUM_OF_CPUS],
        }
    }

    pub fn set_vcpu_config(&mut self, v2p_cpu: [usize; NUM_OF_CPUS]) {
        self.v2p_cpu = v2p_cpu;
    }
}

impl VirtualRegister for EnableReg {
    type Register = u32;
    
    fn write(&mut self, addr: usize, val: u32) {
        let hart_offset = 0x80;    
        let vcpuid = (addr - INT_ENABLE0_CONTEXT0) / hart_offset;
        write_raw(addr + self.v2p_cpu[vcpuid]*hart_offset, val);
    }

    fn read(&mut self, addr: usize) -> u32 {
        let hart_offset = 0x80;    
        let vcpuid = (addr - INT_ENABLE0_CONTEXT0) / hart_offset;
        read_raw(addr + self.v2p_cpu[vcpuid]*hart_offset)
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

    fn write(&mut self, addr: usize, val: u32) {
        if (self.reg == val) {
            self.reg = 0;
        }
        else {
            self.reg = val;
        }
    }

    fn read(&mut self, addr: usize) -> u32 {
        let result = self.reg;
        self.reg = 0;
        result
    }
}

