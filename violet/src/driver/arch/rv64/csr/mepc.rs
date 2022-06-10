//! Machine Exception Program Counter(mepc)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub mepc [
        MEPC       OFFSET(0)  NUMBITS(32) []
    ]
}

#[derive(Clone)]
pub struct Mepc;

impl RegisterReadWrite<u32, mepc::Register> for Mepc {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            asm!("csrr $0, mepc" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            asm!("csrw mepc, $0" :: "r"(value) :: "volatile");
        }
    }
}
