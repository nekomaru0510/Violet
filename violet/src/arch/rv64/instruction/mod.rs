//! RISC-V instructions

use core::arch::asm;
use core::arch::riscv64::hlvx_wu;
use core::ptr::read_unaligned;

pub mod csr;
pub mod env;
pub mod format;
pub mod load;
pub mod ret;
pub mod store;
pub mod wfi;

pub enum Instruction {}

impl Instruction {
    pub fn fetch(addr: usize) -> usize {
        Self::mask(unsafe { read_unaligned(addr as *const usize) })
    }

    pub fn fetch_in_vm(addr: usize) -> usize {
        // Check alignment
        if addr & 0x3 == 0 {
            Self::mask(unsafe { hlvx_wu(addr as *const u32) as usize})
        } else {
            // Emulate unaligned access
            let diff = addr & 0x3;
            let low_addr = addr & 0xffff_ffff_ffff_fffc;
            let high_addr = low_addr + 4;
            let low = unsafe { hlvx_wu(low_addr as *const u32) };
            let high = unsafe { hlvx_wu(high_addr as *const u32) };
            let low = low >> (diff * 8);
            let high_mask: u32 = match diff {
                0 => 0x0000_0000,
                1 => 0x0000_00ff,
                2 => 0x0000_ffff,
                3 => 0x00ff_ffff,
                _ => panic!("Unreachable"),
            };
            let high = ((high & high_mask)) << ((4 - diff) * 8);
            Self::mask((low | high) as usize)
        }
    }

    pub fn len(inst: usize) -> usize {
        if Self::is_compressed(inst) {
            2
        } else {
            4
        }
    }

    pub fn is_compressed(inst: usize) -> bool {
        if inst & (0b11 << 0) == 0b11 {
            false
        } else {
            true
        }
    }

    fn mask(inst: usize) -> usize {
        if Self::is_compressed(inst) {
            inst & 0x0000_ffff
        } else {
            inst & 0xffff_ffff
        }
    }

    pub fn ecall(
        ext: i32,
        fid: i32,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> (usize, usize) {
        let mut val: usize;
        let mut err: usize;
        unsafe {
            asm! ("
            .align 8
                    ecall
            ",
            inout("a0") arg0 => err,
            inout("a1") arg1 => val,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") ext,
        );

            return (err, val);
        }
    }

    pub fn sret(next_addr: usize, arg1: usize, arg2: usize) -> !{
        if next_addr == 0 {
            unsafe {
                asm! ("
                .align 8
                        la  a0, 1f
                        csrw sepc, a0
                        sret
                1:
                        nop
                ",
                options(nostack, noreturn)
                );
            }
        } else {
            unsafe {
                asm! ("
                .align 8
                        csrw sepc, {0}
                        addi a0, {1}, 0
                        addi a1, {2}, 0
                        sret
                ",
                in(reg) next_addr, in(reg) arg1, in(reg) arg2,
                options(nostack, noreturn)
                );
            }
        }
    }

    pub fn wfi() {
        unsafe {
            asm! ("
            .align 8
                    wfi
            ");
        }
    }
}

#[test_case]
fn test_inst() -> Result<(), &'static str> {
    let inst = 0xfcf43423;
    if Instruction::len(inst) == 4 {
        Ok(())
    } else {
        Err("Failed to calc instruction length")
    }
}
