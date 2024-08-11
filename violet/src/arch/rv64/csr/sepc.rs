//! Supervisor Exception Program Counter(sepc)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub sepc [
        SEPC       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Sepc;

impl RegisterReadWrite<u64, sepc::Register> for Sepc {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, sepc" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw sepc, $0" :: "r"(value) :: "volatile");
        }
    }
}
