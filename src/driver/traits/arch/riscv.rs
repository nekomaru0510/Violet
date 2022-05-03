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

/* 特権モード */
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
    SupervisorSoftwareInterrupt=1,
    VirtualSupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    SupervisorTimerInterrupt=5,
    VirtualSupervisorTimerInterrupt,
    MachineTimerInterrupt,
    SupervisorExternalInterrupt=9,
    VirtualSupervisorExternalInterrupt,
    MachineExternalInterrupt,
    SupervisorGuestExternalInterrupt=12,
    //CustomInterrupt(usize),
}

impl Interrupt {
    pub fn mask(&self) -> usize{
        (1 << *self as usize)
    }
}

/* 例外 */
#[derive(Clone, Copy)]
pub enum Exception {
    InstructionAddressMisaligned=0,
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
    LoadPageFault=13,
    StoreAmoPageFault=15,
    InstructionGuestPageFault=20,
    LoadGuestPageFault,
    VirtualInstruction,
    SotreAmoGuestPageFault,
    //CustomException(usize),
}

impl Exception {
    pub fn mask(&self) -> usize{
        (1 << *self as usize)
    }
}

pub enum PagingMode {
    Bare=0,
    Sv39x4=8,
    Sv48x4=9,
    Sv57x4=10,
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

    /* 外部割込みの */
    //fn enable_interrupt_mask(&self, int_mask: usize);
    //fn jump_by_sret(next_addr: usize, arg1: usize, arg2: usize);

    /* 割込みごとの有効化 */
    fn enable_interrupt_mask(&self, int_mask: usize);
    /* 割込みごとの無効化 */ 
    fn disable_interrupt_mask(&self, int_mask: usize);

    /* 外部割込みごとの有効化 */
    fn enable_external_interrupt_mask(&self, int_mask: usize);
    /* 外部割込みごとの無効化 */
    fn disable_external_interrupt_mask(&self, int_mask: usize);

    /* 割込み譲渡の有効化 */
    fn enable_interrupt_delegation_mask(&self, int_mask: usize);
    /* 割込み譲渡の無効化*/
    fn disable_interrupt_delegation_mask(&self, int_mask: usize);

    /* 例外譲渡の有効化(HS-mode用) */
    fn enable_exception_delegation_mask(&self, exc_mask: usize);
    /* 例外譲渡の無効化(HS-mode用) */
    fn disable_exception_delegation_mask(&self, exc_mask: usize);

    /* VS-modeの割込みフラッシュ */
    fn flush_vsmode_interrupt(&self);
    /* VS-modeへの割込み生成 */
    fn assert_vsmode_interrupt(&self, int_mask:usize);
    /* VS-modeの各種レジスタアクセスの許可 */
    fn enable_vsmode_counter_access(&self, counter_mask:usize);
    /* VS-modeの各種レジスタアクセスの不許可 */
    fn disable_vsmode_counter_access(&self, counter_mask:usize);

    /* ページングモードの設定 */
    fn set_paging_mode(&self, mode: PagingMode);
}
