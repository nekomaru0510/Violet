//! RISC-V用のトレイト

/* 割込み・例外元のコンテキストを示す */
#[derive(Clone, Copy)]
pub struct Registers {
    pub zero: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize, //fp
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub epc: usize,
}

/* 特権モード */
#[derive(Clone, Copy)]
pub enum PrivilegeMode {
    ModeM,
    ModeHS,
    ModeS,
    ModeHU,
    ModeU,
    ModeVS,
    ModeVU,
}

/* 割込み */
#[derive(Clone, Copy)]
pub enum Interrupt {
    SupervisorSoftwareInterrupt = 1,
    VirtualSupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    SupervisorTimerInterrupt = 5,
    VirtualSupervisorTimerInterrupt,
    MachineTimerInterrupt,
    SupervisorExternalInterrupt = 9,
    VirtualSupervisorExternalInterrupt,
    MachineExternalInterrupt,
    SupervisorGuestExternalInterrupt = 12,
    //CustomInterrupt(usize),
}

impl Interrupt {
    pub fn mask(&self) -> usize {
        (1 << *self as usize)
    }
}

/* 例外 */
#[derive(Clone, Copy)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAmoAddressMisaligned,
    StoreAmoAccessFault,
    EnvironmentCallFromUmodeOrVUmode,
    EnvironmentCallFromHSmode,
    EnvironmentCallFromVSmode,
    EnvironmentCallFromMmode,
    InstructionPageFault,
    LoadPageFault = 13,
    StoreAmoPageFault = 15,
    InstructionGuestPageFault = 20,
    LoadGuestPageFault,
    VirtualInstruction,
    SotreAmoGuestPageFault,
    //CustomException(usize),
}

impl Exception {
    pub fn mask(&self) -> usize {
        (1 << *self as usize)
    }
}

pub enum PagingMode {
    Bare = 0,
    Sv39x4 = 8,
    Sv48x4 = 9,
    Sv57x4 = 10,
}

pub trait TraitRisvCpu {
    /* 割込みの登録 */
    fn register_interrupt(&self, int_num: usize, func: fn(int_num: usize, regs: &mut Registers));
    /* 例外の登録 */
    fn register_exception(&self, exc_num: usize, func: fn(exc_num: usize, regs: &mut Registers));

    /* HS-modeへの切替え */
    fn switch_hs_mode(&self);
    /* 次の特権モードの設定 */
    fn set_next_mode(&self, mode: PrivilegeMode);
}


