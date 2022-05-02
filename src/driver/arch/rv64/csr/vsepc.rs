//! Virtual Supervisor Exception Program Counter(vsepc)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub vsepc [
        VSEPC       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Vsepc;

impl RegisterReadWrite<u64, vsepc::Register> for Vsepc {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x241" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x241, $0" :: "r"(value) :: "volatile");
        }
    }
}
