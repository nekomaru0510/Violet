//! Hypervisor Counter-Enable Register (hcounteren)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub hcounteren [
        HCOUNTEREN       OFFSET(0)  NUMBITS(32) []
    ]
}

#[derive(Clone)]
pub struct Hcounteren;

impl RegisterReadWrite<u32, hcounteren::Register> for Hcounteren {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x606" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            asm!("csrw 0x606, $0" :: "r"(value) :: "volatile");
        }
    }
}
