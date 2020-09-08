//! Machine Cause Register

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub mcause [
        EXCEPTION_CODE  OFFSET(0)  NUMBITS(31) [
            USER_SOFTWARE = 0,
            SUPERVISOR_SOFTWARE = 1,
            MACHINE_SOFTWARE = 3,
            USER_TIMER = 4,
            SUPERVISOR_TIMER = 5,
            MACHINE_TIMER = 7,
            USER_EXTERNAL = 8,
            SUPERVISOR_EXTERNAL = 9,
            MACHINE_EXTERNAL = 11
        ],
        INTERRUPT       OFFSET(30)  NUMBITS(1) []
    ]
}

pub struct Mcause;

impl RegisterReadWrite<u32, mcause::Register> for Mcause {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            llvm_asm!("csrr $0, mcause" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            llvm_asm!("csrw mcause, $0" :: "r"(value) :: "volatile");
        }
    }
}