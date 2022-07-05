//! RISC-V用命令

#[derive(Clone)]
pub struct Rv64Inst {}

impl Rv64Inst {
    pub const fn new() -> Self {
        Rv64Inst {}
    }

    pub fn do_ecall(
        &self,
        ext: i32,
        fid: i32,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> (usize, usize) {
        unsafe {
            let mut val: usize = 0;
            let mut err: usize = 0;

            asm! ("
            .align 8
                    addi a0, $2, 0
                    addi a1, $3, 0
                    addi a2, $4, 0
                    addi a3, $5, 0
                    addi a4, $6, 0
                    addi a5, $7, 0
                    addi a6, $8, 0
                    addi a7, $9, 0
                    ecall
                    addi $0, a0, 0
                    addi $1, a1, 0
            "
            : "+r"(err), "+r"(val)
            : "r"(arg0), "r"(arg1), "r"(arg2), "r"(arg3), "r"(arg4), "r"(arg5), "r"(fid), "r"(ext)
            : "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"
            : );

            return (err, val);
        }
    }

    pub fn jump_by_sret(&self, next_addr: usize, arg1: usize, arg2: usize) {
        if next_addr == 0 {
            unsafe {
                asm! ("
                .align 8
                        la  a0, 1f
                        csrw sepc, a0
                        sret
                1:
                        nop
                "
                :
                :
                :
                : "volatile");
            }
        } else {
            unsafe {
                asm! ("
                .align 8
                        csrw sepc, $0
                        addi a0, $1, 0
                        addi a1, $2, 0
                        sret
                "
                :
                : "r"(next_addr), "r"(arg1), "r"(arg2) 
                :
                : "volatile");
            }
        }
    }
}
