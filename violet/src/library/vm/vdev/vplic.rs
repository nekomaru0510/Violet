//! Virtual PLIC

use core::usize;

use super::VirtualDeviceT;
use super::{read_raw, write_raw};
use crate::arch::rv64::extension::hypervisor::Hext;
use crate::arch::rv64::trap::int::Interrupt;
use crate::arch::rv64::PrivilegeMode;
use crate::arch::traits::TraitArch;
use crate::environment::{Arch, NUM_OF_CPUS};
use crate::resource::{get_resources, BorrowResource, ResourceType}; /* [todo delete] */

const NUM_OF_IRQS: usize = 1024;
const NUM_OF_CONTEXTS: usize = NUM_OF_CPUS * 2;

#[repr(C)]
#[repr(align(4096))]
pub struct VPlic {
    pending_count: usize,
    priority: [u32; NUM_OF_IRQS],
    pending: [u32; NUM_OF_IRQS / 32],
    claimed: [[u32; NUM_OF_IRQS / 32]; NUM_OF_CONTEXTS],
    enable: [[u32; NUM_OF_IRQS / 32]; NUM_OF_CONTEXTS],
    priority_threshold: [u32; NUM_OF_CONTEXTS],
    v2p_cpu: [usize; NUM_OF_CPUS],
    p2v_cpu: [usize; NUM_OF_CPUS],
    mode: PrivilegeMode,
}


const INT_PRIORITY0: usize = 0x0;
const INT_PRIORITY1023: usize = 0xffc;
const INT_PENDING0: usize = 0x1000;
const INT_PENDING992: usize = 0x107c;
const INT_ENABLE0_HART_OFFSET: usize = 0x100;
const INT_ENABLE0_CONTEXT_OFFSET: usize = 0x80;
const INT_ENABLE0_CONTEXT0: usize = 0x2000; /* Hart 0 (M-mode) */
const INT_ENABLE992_CONTEXT0: usize = 0x207c;
const INT_ENABLE0_CONTEXT1: usize = 0x2080; /* Hart 0 (S-mode) */
const INT_ENABLE992_CONTEXT1: usize = 0x20fc;
const INT_ENABLE0_CONTEXT15871: usize = 0x1f_1f80; /* Hart 7935 (S-mode) */
const INT_ENABLE992_CONTEXT15871: usize = 0x1f_1ffc;
const PRIO_THRESHOLD_HART_OFFSET: usize = 0x2000;
const PRIO_THRESHOLD_CONTEXT_OFFSET: usize = 0x1000;
const PRIO_THRESHOLD_CONTEXT0: usize = 0x20_0000; /* Hart 0 (M-mode) */
const PRIO_THRESHOLD_CONTEXT1: usize = 0x20_1000; /* Hart 0 (S-mode) */
const PRIO_THRESHOLD_CONTEXT3: usize = PRIO_THRESHOLD_CONTEXT1 + PRIO_THRESHOLD_HART_OFFSET;
const PRIO_THRESHOLD_CONTEXT15871: usize = 0x3ff_f000; /* Hart 7935 (S-mode) */
const CLAIM_COMPLETE_HART_OFFSET: usize = 0x2000;
const CLAIM_COMPLETE_CONTEXT_OFFSET: usize = 0x1000;
const CLAIM_COMPLETE_CONTEXT0: usize = 0x20_0004; /* Hart 0 (M-mode) */
const CLAIM_COMPLETE_CONTEXT1: usize = 0x20_1004; /* Hart 0 (S-mode) */
const CLAIM_COMPLETE_CONTEXT3: usize = CLAIM_COMPLETE_CONTEXT1 + CLAIM_COMPLETE_HART_OFFSET;
const CLAIM_COMPLETE_CONTEXT15871: usize = 0x3ff_f004; /* Hart 7935 (S-mode) */

const BASE_ADDRESS: usize = 0xC00_0000; /* [todo delete] */
const ADDRESS_RANGE: usize = 0x400_0000;
const MASK: usize = 0x3ff_ffff;

impl VPlic {
    pub const fn new() -> Self {
        VPlic {
            pending_count: 0,
            priority: [0; NUM_OF_IRQS],
            pending: [0; NUM_OF_IRQS / 32],
            claimed: [[0; NUM_OF_IRQS / 32]; NUM_OF_CONTEXTS],
            enable: [[0; NUM_OF_IRQS / 32]; NUM_OF_CONTEXTS],
            priority_threshold: [0; NUM_OF_CONTEXTS],
            v2p_cpu: [0; NUM_OF_CPUS],
            p2v_cpu: [usize::MAX; NUM_OF_CPUS],
            mode: PrivilegeMode::ModeS,
        }
    }

    pub fn set_vcpu_config(&mut self, v2p_cpu: [usize; NUM_OF_CPUS]) {
        self.v2p_cpu = v2p_cpu;
        for idx in 0..NUM_OF_CPUS {
            let pidx = self.v2p_cpu[idx];
            if pidx >= NUM_OF_CPUS {
                continue;
            }
            self.p2v_cpu[self.v2p_cpu[idx]] = idx;
        }
    }

    /* Setup for vplic for virtual machine mode */
    pub fn enable_vmmode(&mut self) {
        self.mode = PrivilegeMode::ModeM;
    }

    fn priority_write(&mut self, addr: usize, val: u32) {
        let idx = (addr - BASE_ADDRESS) / 4;
        self.priority[idx] = val;
        write_raw(addr, val);
        self.update_state();
    }

    fn priority_read(&mut self, addr: usize) -> u32 {
        let idx = (addr - BASE_ADDRESS) / 4;
        self.priority[idx]
    }

    fn pending_write(&mut self, addr: usize, val: u32) {
        return; // Read only
    }

    fn pending_read(&mut self, addr: usize) -> u32 {
        let word_idx = (addr - BASE_ADDRESS - INT_PENDING0) / 4;
        self.pending[word_idx] as u32
    }

    fn enable_write(&mut self, addr: usize, val: u32) {
        let context = (addr - BASE_ADDRESS - INT_ENABLE0_CONTEXT0) / INT_ENABLE0_CONTEXT_OFFSET;
        let vcpuid = context / 2;
        let word_idx = (addr - BASE_ADDRESS - INT_ENABLE0_CONTEXT0 - INT_ENABLE0_CONTEXT_OFFSET * context) / 4;

        self.enable[context][word_idx] = val;

        /* Set all interrupts to trigger into HS-mode */
        write_raw(
            BASE_ADDRESS + INT_ENABLE0_CONTEXT0
                + self.v2p_cpu[vcpuid] * INT_ENABLE0_HART_OFFSET
                + INT_ENABLE0_CONTEXT_OFFSET
                + word_idx * 4,
            val
        );
    }

    fn enable_read(&mut self, addr: usize) -> u32 {
        let context = (addr - BASE_ADDRESS - INT_ENABLE0_CONTEXT0) / INT_ENABLE0_CONTEXT_OFFSET;
        let word_idx = (addr - BASE_ADDRESS - INT_ENABLE0_CONTEXT0 - INT_ENABLE0_CONTEXT_OFFSET * context) / 4;

        self.enable[context][word_idx]
    }

    fn priority_threshold_write(&mut self, addr: usize, val: u32) {
        let context = (addr - BASE_ADDRESS - PRIO_THRESHOLD_CONTEXT0) / PRIO_THRESHOLD_CONTEXT_OFFSET;
        let vcpuid = context / 2;
        self.priority_threshold[context] = val;

        /* Set all interrupts to trigger into HS-mode */
        write_raw(
            BASE_ADDRESS + PRIO_THRESHOLD_CONTEXT0
                + self.v2p_cpu[vcpuid] * PRIO_THRESHOLD_HART_OFFSET
                + PRIO_THRESHOLD_CONTEXT_OFFSET,
            val,
        );
        self.update_state();
    }

    fn priority_threshold_read(&mut self, addr: usize) -> u32 {
        let context = (addr - BASE_ADDRESS - PRIO_THRESHOLD_CONTEXT0) / PRIO_THRESHOLD_CONTEXT_OFFSET;
        self.priority_threshold[context]
    }

    fn claim_comp_write(&mut self, addr: usize, val: u32) {
        if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
            i.set_comp_int(val);
        }
        let context = (addr - BASE_ADDRESS - CLAIM_COMPLETE_CONTEXT0) / CLAIM_COMPLETE_CONTEXT_OFFSET;
        // Clear claimed bit of the interrupt
        self.clear_claimed(val, context);
        self.update_state();
    }

    fn claim_comp_read(&mut self, addr: usize) -> u32 {
        let context = (addr - BASE_ADDRESS - CLAIM_COMPLETE_CONTEXT0) / CLAIM_COMPLETE_CONTEXT_OFFSET;

        // Get the highest priority interrupt
        let result = self.get_next_pending(context);

        if result == 0 {
            return 0;
        }

        // Clear pending bit of the interrupt
        self.clear_pending(result);
        // Set claimed bit of the interrupt
        self.set_claimed(result, context);
        
        self.update_state();
    
        result
    }

    fn claim_comp_int(&mut self, intid: u32) {
        self.set_pending(intid);
        self.update_state();
    }

    fn set_pending(&mut self, intid: u32) {
        let word_idx = (intid / 32) as usize;
        let word_bit = (intid % 32) as usize;
        self.pending[word_idx] |= 1 << word_bit;
    }

    fn clear_pending(&mut self, intid: u32) {
        let word_idx = (intid / 32) as usize;
        let word_bit = (intid % 32) as usize;
        self.pending[word_idx] &= !(1 << word_bit);
    }

    fn set_claimed(&mut self, intid: u32, context: usize) {
        let word_idx = (intid / 32) as usize;
        let word_bit = (intid % 32) as usize;
        self.claimed[context][word_idx] |= 1 << word_bit;
    }

    fn clear_claimed(&mut self, intid: u32, context: usize) {
        let word_idx = (intid / 32) as usize;
        let word_bit = (intid % 32) as usize;
        self.claimed[context][word_idx] &= !(1 << word_bit);
    }

    // Get max priority interrupt
    fn get_next_pending(&mut self, context: usize) -> u32 {
        let mut intid = 0;
        let mut max_prio = self.priority_threshold[context];

        for word_idx in 0..NUM_OF_IRQS / 32 {
            let pending = self.pending[word_idx];
            let claimed = self.claimed[context][word_idx];
            let enable = self.enable[context][word_idx];

            let candidates = (pending & !claimed) & enable;
            
            if candidates == 0 {
                continue;
            }

            for i in 0..32 {
                let irq = word_idx * 32 + i;
                let is_enabled = (candidates & (1 << i)) > 0;
                let prio = self.priority[irq];

                if is_enabled && prio > max_prio {
                    intid = irq as u32;
                    max_prio = prio;
                }
            }
        }

        intid as u32
    }

    /* Call when priority, pending, priotity_threshold, claimed are changed */
    fn update_state(&mut self) {
        // Currently, only either M-mode or S-mode is supported
        let context = match self.mode {
            // M-mode
            PrivilegeMode::ModeM => self.p2v_cpu[Arch::get_cpuid()] * 2,
            // S-mode
            PrivilegeMode::ModeS => self.p2v_cpu[Arch::get_cpuid()] * 2 + 1,
            _ => return,
        };
        
        let intid = self.get_next_pending(context);
        if intid > 0 {
            // Assert the interrupt to the VCPU
            Hext::assert_vsmode_interrupt(
                Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT),
            );
        } else {
            // Flush the interrupt to the VCPU
            Hext::flush_vsmode_interrupt(
                Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT),
            );
        }
    }
}

impl VirtualDeviceT for VPlic {
    fn write(&mut self, addr: usize, val: usize) {
        // [todo fix] Consolidate register acquisition into a function
        match addr & MASK {
            INT_PRIORITY0..=INT_PRIORITY1023 => self.priority_write(addr, val as u32),
            INT_PENDING0..=INT_PENDING992 => self.pending_write(addr, val as u32),
            INT_ENABLE0_CONTEXT0..=INT_ENABLE992_CONTEXT15871 => self.enable_write(addr, val as u32),
            PRIO_THRESHOLD_CONTEXT0..=CLAIM_COMPLETE_CONTEXT15871 => {
                /* CONTEXT */
                if addr & 0x0fff == 0 {
                    self.priority_threshold_write(addr, val as u32)
                } else {
                    self.claim_comp_write(addr, val as u32)
                }
            }
            _ => (),
        };
    }

    fn read(&mut self, addr: usize) -> usize {
        let ret = match addr & MASK {
            INT_PRIORITY0..=INT_PRIORITY1023 => self.priority_read(addr),
            INT_PENDING0..=INT_PENDING992 => self.pending_read(addr),
            INT_ENABLE0_CONTEXT0..=INT_ENABLE992_CONTEXT15871 => self.enable_read(addr),
            PRIO_THRESHOLD_CONTEXT0..=CLAIM_COMPLETE_CONTEXT15871 => {
                /* CONTEXT */
                if addr & 0x0fff == 0 {
                    self.priority_threshold_read(addr)
                } else {
                    self.claim_comp_read(addr)
                }
            }
            _ => 0,
        };
        ret as usize
    }

    fn interrupt(&mut self, intid: usize) {
        self.claim_comp_int(intid as u32);
    }
}
