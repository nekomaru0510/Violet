//! RISC-V用のトレイト

/* 割込み・例外元のコンテキストを示す */
#[derive(Clone, Copy)]
pub struct Registers {
    pub gp : [usize; 16], /* 汎用レジスタ */
    pub epc: usize,
    cause: usize,
    tval: usize,
    scratch: usize,
    sp : *mut usize,
}

pub trait TraitRisvCpu {
    /* 割込みの登録 */
    fn register_interrupt(&self, int_num: usize, func: fn(int_num: usize, regs: &mut Registers));
    /* 例外の登録 */
    fn register_exception(&self, exc_num: usize, func: fn(exc_num: usize, regs: &mut Registers));
}

/*
impl Registers {
    pub fn empty() -> Registers {
        Registers {epc: 0, cause: 0, tval: 0, scrach: 0, regs: [0;32], sp: 0 as *mut usize, }
    }

    pub fn new(sp: *mut usize) -> Registers {
        Registers {epc: 0, cause: 0, tval: 0, scrach: 0, regs: [0;32], sp: 0 as *mut usize, }
        /*
        let regs: [usize; 32] = ;
        Registers {
            epc: (*(sp)).reg[16], 
            cause: (*(sp)).reg[17], 
            tval: (*(sp)).reg[18], 
            scrach: (*(sp)).reg[19], 
            regs: (*(sp)).reg, 
            sp,
        } */
    }
}
 */