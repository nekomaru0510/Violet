//! Hypervisor interrupt-pending register(hip)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub hip [
        VSSIP     OFFSET(2)  NUMBITS(1) [],
        VSTIP     OFFSET(6)  NUMBITS(1) [],
        VSEIP     OFFSET(10) NUMBITS(1) [],
        SGEIE     OFFSET(12) NUMBITS(1) []
    ]
}

#[derive(Clone)]
pub struct Hip;

impl RegisterReadWrite<u32, hip::Register> for Hip {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x644" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            asm!("csrw 0x644, $0" :: "r"(value) :: "volatile");
        }
    }
}


