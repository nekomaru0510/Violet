//! Sifive Platform-Level Interrupt Controller(PLIC)

use core::ptr::{write_volatile/*, read_volatile*/};
use crate::kernel::resource::irq::IrqAttr;

pub struct Plic {
    base: usize, /* 0x0C00_0000 */
}

//const SOURCE_1_PRIO: usize = 0x4;
//const START_OF_PENDING_ARRAY: usize = 0x1000;
const START_HART0_INT_ENABLE: usize = 0x2000;
//const HART0_PRIO_THRESHOLD: usize = 0x2_0000;
//const HART0_CLAIM_COMPLETE: usize = 0x2_0004;

impl IrqAttr for Plic {
    fn new() -> Self {
        Plic {base: 0x0c00_0000,}
    }

    fn enable_interrupt(&self, id: u64) {
        self.set_enable(id);
    }

}

impl Plic {
    pub fn set_enable(&self, id:u64) {
        let offset = ((id / 32)*4) as usize;
        let val = 0x01 << (id % 32) as u32;

        unsafe {
            write_volatile((self.base + START_HART0_INT_ENABLE + offset) as *mut u64, val);
        }
    }

}

