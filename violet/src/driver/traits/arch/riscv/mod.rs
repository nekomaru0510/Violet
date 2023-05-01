//! RISC-V用のトレイト

use crate::driver::arch::rv64::regs::Registers; // [todo delete]

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
        1 << *self as usize
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
    StoreAmoGuestPageFault,
    //CustomException(usize),
}

impl Exception {
    pub fn mask(&self) -> usize {
        1 << *self as usize
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
    fn register_interrupt(&self, int_num: Interrupt, func: fn(regs: &mut Registers));
    /* 例外の登録 */
    fn register_exception(&self, exc_num: Exception, func: fn(regs: &mut Registers));

    /* HS-modeへの切替え */
    fn switch_hs_mode(&self);
    /* 次の特権モードの設定 */
    fn set_next_mode(&self, mode: PrivilegeMode);
}
