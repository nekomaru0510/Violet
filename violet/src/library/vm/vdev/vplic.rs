//! 仮想PLIC

use super::VirtualDeviceT;
use super::{read_raw, write_raw};
use crate::arch::rv64::get_cpuid; // [todo delete] //test
                                          //use crate::driver::traits::intc::TraitIntc;
use crate::environment::NUM_OF_CPUS;
use crate::resource::{get_resources, BorrowResource, ResourceType}; /* [todo delete] */

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    priority_threshold: u32,
    claim_comp: [u32; NUM_OF_CPUS],
    //interrupt: [InterruptState; 64],
    v2p_cpu: [usize; NUM_OF_CPUS],
    p2v_cpu: [usize; NUM_OF_CPUS],
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

const INT_ENABLE0_HART_OFFSET: usize = 0x100; //0x80;
const INT_ENABLE0_CONTEXT0: usize = 0x2080;
const PRIO_THRESHOLD_HART_OFFSET: usize = 0x2000; //0x1000;
const PRIO_THRESHOLD_CONTEXT0: usize = 0x20_1000; //0x20_0000;//0x20_1000;
const PRIO_THRESHOLD_CONTEXT1: usize = PRIO_THRESHOLD_CONTEXT0 + PRIO_THRESHOLD_HART_OFFSET;
const CLAIM_COMPLETE_HART_OFFSET: usize = 0x2000; //0x1000;
const CLAIM_COMPLETE_CONTEXT0: usize = 0x20_1004; //0x20_0004;//0x20_1004;
const CLAIM_COMPLETE_CONTEXT1: usize = CLAIM_COMPLETE_CONTEXT0 + CLAIM_COMPLETE_HART_OFFSET;

const BASE_ADDRESS: usize = 0xC00_0000; /* [todo delete] */
const ADDRESS_RANGE: usize = 0x400_0000;
const MASK: usize = 0x3ff_ffff; /* [todo fix] 上記要素から算出できるように */

impl VPlic {
    pub const fn new() -> Self {
        VPlic {
            //priority_threshold: PriorityThresholdReg::new(),
            //claim_comp: ClaimCompReg::new(),
            priority_threshold: 0,
            claim_comp: [0; NUM_OF_CPUS],
            v2p_cpu: [0; NUM_OF_CPUS],
            p2v_cpu: [0; NUM_OF_CPUS],
        }
    }

    pub fn set_vcpu_config(&mut self, v2p_cpu: [usize; NUM_OF_CPUS]) {
        self.v2p_cpu = v2p_cpu;
        for idx in 0..NUM_OF_CPUS {
            let pidx = self.v2p_cpu[idx];
            if pidx > NUM_OF_CPUS {
                continue;
            }
            self.p2v_cpu[self.v2p_cpu[idx]] = idx;
        }
    }

    fn enable_write(&mut self, addr: usize, val: u32) {
        let hart_offset = INT_ENABLE0_HART_OFFSET;
        let vcpuid = (addr - BASE_ADDRESS - INT_ENABLE0_CONTEXT0) / hart_offset;
        write_raw(addr + self.v2p_cpu[vcpuid] * hart_offset, val);
    }

    fn enable_read(&mut self, addr: usize) -> u32 {
        let hart_offset = INT_ENABLE0_HART_OFFSET;
        let vcpuid = (addr - BASE_ADDRESS - INT_ENABLE0_CONTEXT0) / hart_offset;
        read_raw(addr + self.v2p_cpu[vcpuid] * hart_offset)
    }

    fn priority_threshold_write(&mut self, addr: usize, val: u32) {
        let hart_offset = PRIO_THRESHOLD_HART_OFFSET;
        let vcpuid = (addr - BASE_ADDRESS - PRIO_THRESHOLD_CONTEXT0) / hart_offset;
        write_raw(
            BASE_ADDRESS + PRIO_THRESHOLD_CONTEXT0 + self.v2p_cpu[vcpuid] * hart_offset,
            val,
        );
    }

    fn priority_threshold_read(&mut self, addr: usize) -> u32 {
        let hart_offset = PRIO_THRESHOLD_HART_OFFSET;
        let vcpuid = (addr - BASE_ADDRESS - PRIO_THRESHOLD_CONTEXT0) / hart_offset;
        read_raw(BASE_ADDRESS + PRIO_THRESHOLD_CONTEXT0 + self.v2p_cpu[vcpuid] * hart_offset)
    }

    fn claim_comp_write(&mut self, addr: usize, val: u32) {}

    fn claim_comp_read(&mut self, addr: usize) -> u32 {
        let hart_offset = CLAIM_COMPLETE_HART_OFFSET;
        let vcpuid = (addr - BASE_ADDRESS - CLAIM_COMPLETE_CONTEXT0) / hart_offset;

        let result = self.claim_comp[vcpuid];

        self.claim_comp[vcpuid] =
            if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
                i.get_pend_int()
            } else {
                0
            };

        result
    }

    fn claim_comp_int(&mut self, intid: u32) {
        let vcpuid = self.p2v_cpu[get_cpuid()];
        self.claim_comp[vcpuid] = intid as u32;
    }
}

impl VirtualDeviceT for VPlic {
    fn write(&mut self, addr: usize, val: usize) {
        /* [todo fix] レジスタ取得を関数にまとめたい */
        match addr & MASK {
            INT_ENABLE0_CONTEXT0 => self.enable_write(addr, val as u32),
            PRIO_THRESHOLD_CONTEXT0 => self.priority_threshold_write(addr, val as u32),
            PRIO_THRESHOLD_CONTEXT1 => self.priority_threshold_write(addr, val as u32),
            CLAIM_COMPLETE_CONTEXT0 => self.claim_comp_write(addr, val as u32),
            _ => write_raw(addr, val as u32),
        };
    }

    fn read(&mut self, addr: usize) -> usize {
        let ret = match addr & MASK {
            INT_ENABLE0_CONTEXT0 => self.enable_read(addr),
            PRIO_THRESHOLD_CONTEXT0 => self.priority_threshold_read(addr),
            PRIO_THRESHOLD_CONTEXT1 => self.priority_threshold_read(addr),
            CLAIM_COMPLETE_CONTEXT0 => self.claim_comp_read(addr),
            _ => read_raw(addr),
        };
        ret as usize
    }

    fn interrupt(&mut self, intid: usize) {
        self.claim_comp_int(intid as u32);
    }
}

/*
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
        let con = get_mut_container();
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
*/
