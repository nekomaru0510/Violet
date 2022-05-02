//! RISC-V用のトレイト

/* 割込み・例外元のコンテキストを示す */
#[derive(Clone, Copy)]
pub struct Registers {
    epc: usize,
    regs : [usize; 32],
    sp : *mut usize,
    regsize: u32,
}

pub trait TraitRisvCpu {
    /* 割込みの登録 */
    fn register_interrupt(&self, int_num: usize, func: fn(int_num: usize, regs: Registers));
    /* 例外の登録 */
    fn register_exception(&self, exc_num: usize, func: fn(int_num: usize, regs: Registers));
}

impl Registers {
    pub fn new() -> Registers {
        Registers {epc: 0, regs: [0;32], sp: 0 as *mut usize, regsize: 0, }
    }
}