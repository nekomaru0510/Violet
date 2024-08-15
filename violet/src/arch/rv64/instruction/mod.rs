//! RISC-V用命令
use core::arch::asm;
//use core::ptr::read_volatile;
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
        let mut val: usize = 0;
        let mut err: usize = 0;
        unsafe {
            asm! ("
            .align 8
                    addi a0, {2}, 0
                    addi a1, {3}, 0
                    addi a2, {4}, 0
                    addi a3, {5}, 0
                    addi a4, {6}, 0
                    addi a5, {7}, 0
                    addi a6, {8}, 0
                    addi a7, {9}, 0
                    ecall
                    addi {0}, a0, 0
                    addi {1}, a1, 0
            ",
            out(reg) err, out(reg) val, //"+r"(err), "+r"(val),
            //"r"(arg0), "r"(arg1), "r"(arg2), "r"(arg3), "r"(arg4), "r"(arg5), "r"(fid), "r"(ext),
            in(reg) arg0, in(reg) arg1, in(reg) arg2, in(reg) arg3, in(reg) arg4, in(reg) arg5, in(reg) fid, in(reg) ext,  
            //"a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7",
            options(nostack)
        );

            return (err, val);
        }
    }

    pub fn sret(next_addr: usize, arg1: usize, arg2: usize) {
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
                options(nostack)
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
                //"r"(next_addr), "r"(arg1), "r"(arg2) 
                in(reg) next_addr, in(reg) arg1, in(reg) arg2,
                options(nostack)
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
