//! RV64I CPU ドライバ

/*  */
use crate::environment::STACK_SIZE;

/* ドライバ用トレイト */
use crate::driver::traits::cpu::TraitCpu;

pub mod regs;
use regs::Registers;

pub mod boot;
use boot::_start_trap;

pub mod mmu;
use mmu::Rv64Mmu;

pub mod inst;
use inst::Rv64Inst;

pub mod int;
use int::Rv64Int;

pub mod exc;
use exc::Rv64Exc;

pub mod hyp;
use hyp::Rv64Hyp;

pub mod sbi;

extern crate register;
use register::cpu::RegisterReadWrite;

extern crate alloc;
extern crate core;
use core::intrinsics::transmute;

pub mod csr;
use csr::hstatus::*;
use csr::scause::*;
use csr::sepc::*;
use csr::sstatus::*;
use csr::stval::*;
use csr::vscause::*;
use csr::vsepc::*;
use csr::vsie::*;
use csr::vsstatus::*;
use csr::vstval::*;
use csr::vstvec::*;
use csr::Csr;

#[derive(Clone)]
pub struct Rv64 {
    pub id: u64,             /* CPUのid */
    pub status: CpuStatus,   /* 状態 */
    pub mode: PrivilegeMode, /* 動作モード */
    pub csr: Csr,            /* CSR [todo delete]*/
    pub inst: Rv64Inst,
    pub int: Rv64Int,
    pub exc: Rv64Exc,
    pub mmu: Rv64Mmu,
    pub hyp: Rv64Hyp,
    pub scratch: Scratch, /* scratchレジスタが指す構造体 */
}

#[derive(Clone)]
pub enum CpuStatus {
    STOPPED = 0x00, /* 停止中(Violetとしても管理できてない) */
    STARTED,        /* 起動中 */
    SUSPENDED,      /* 停止中(Violetが管理している) */
}

// scratchレジスタが指す構造体
#[derive(Copy, Clone)]
pub struct Scratch {
    cpu_id: u64,
    sp: usize,
    tmp0: usize,
    stack_size: usize,
}

impl Scratch {
    pub const fn new(cpu_id: u64) -> Self {
        Scratch {
            cpu_id,
            sp: 0x0,
            tmp0: 0x0,
            stack_size: STACK_SIZE,
        }
    }

    pub fn set_cpu_id(&mut self, cpu_id: u64) {
        self.cpu_id = cpu_id;
    }

    pub fn get_cpu_id(&self) -> u64 {
        self.cpu_id
    }
}

////////////////////////////////
/* ハードウェア依存の機能の実装 */
///////////////////////////////
impl Rv64 {
    pub const fn new(id: u64) -> Self {
        Rv64 {
            id,
            status: CpuStatus::STARTED,
            mode: PrivilegeMode::ModeS,
            csr: Csr::new(),
            inst: Rv64Inst::new(),
            int: Rv64Int::new(),
            exc: Rv64Exc::new(),
            mmu: Rv64Mmu::new(),
            hyp: Rv64Hyp::new(),
            scratch: Scratch::new(id),
        }
    }

    pub fn set_sscratch(&self) {
        unsafe {
            self.csr.sscratch.set(transmute(&self.scratch));
        }
    }

    pub fn set_default_vector(&self) {
        self.set_vector(_start_trap as usize);
    }

    pub fn set_vector(&self, addr: usize) {
        match self.mode {
            PrivilegeMode::ModeM => {
                self.csr.mtvec.set(addr as u64);
            }
            PrivilegeMode::ModeS => {
                self.csr.stvec.set(addr as u64);
            }
            _ => {}
        }
    }
}

//////////////////////////////////////
/* (一般的な)CPUとして必要な機能の実装 */
//////////////////////////////////////
impl TraitCpu for Rv64 {
    fn core_init(&self) {
        self.set_sscratch();
        self.set_default_vector();
        self.enable_interrupt();
    }

    fn wakeup(&self) {
        sbi::sbi_hart_start(self.id, boot::_start_ap as u64, 0xabcd);
    }

    fn sleep(&self) {
        sbi::sbi_hart_stop();
    }

    fn enable_interrupt(&self) {
        self.int.enable_s();
    }

    fn disable_interrupt(&self) {
        self.int.disable_s();
    }
}

const NUM_OF_INTERRUPTS: usize = 32;
const NUM_OF_EXCEPTIONS: usize = 32;

pub static mut INTERRUPT_HANDLER: [Option<fn(regs: &mut Registers)>; NUM_OF_INTERRUPTS] =
    [None; NUM_OF_INTERRUPTS];
pub static mut EXCEPTION_HANDLER: [Option<fn(regs: &mut Registers)>; NUM_OF_EXCEPTIONS] =
    [None; NUM_OF_EXCEPTIONS];

////////////////////////////////
/* アーキテクチャ依存機能の実装 */
///////////////////////////////
impl TraitRisvCpu for Rv64 {
    fn register_interrupt(&self, int_num: Interrupt, func: fn(regs: &mut Registers)) {
        unsafe {
            INTERRUPT_HANDLER[int_num as usize] = Some(func);
        }
    }

    fn register_exception(&self, exc_num: Exception, func: fn(regs: &mut Registers)) {
        unsafe {
            EXCEPTION_HANDLER[exc_num as usize] = Some(func);
        }
    }

    fn switch_hs_mode(&self) {
        /* 次の動作モードをHS-modeに */
        self.set_next_mode(PrivilegeMode::ModeHS);
        /* 次の動作モードへ切替え */
        self.inst.jump_by_sret(0, 0, 0);
    }

    fn set_next_mode(&self, mode: PrivilegeMode) {
        match mode {
            PrivilegeMode::ModeS => {
                self.csr.sstatus.modify(sstatus::SPP::SET);
                self.csr.hstatus.modify(hstatus::SPV::CLEAR);
            }
            PrivilegeMode::ModeVS => {
                self.csr.sstatus.modify(sstatus::SPP::SET);
                self.csr.hstatus.modify(hstatus::SPV::SET);
                self.csr.hstatus.modify(hstatus::SPVP::SET);
            }
            PrivilegeMode::ModeHS => {
                self.csr.sstatus.modify(sstatus::SPP::SET);
                self.csr.hstatus.modify(hstatus::SPV::CLEAR);
            }
            _ => (),
        };
    }
}

////////////////////////////////
/* 関数(アセンブリから飛んでくる関数) */
////////////////////////////////

/* カーネルの起動処理 */
use crate::kernel::boot_init;

// CPU初期化処理 ブート直後に実行される
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn setup_cpu(cpu_id: usize) {
    boot_init(cpu_id);
}

// 割込み・例外ハンドラ
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn trap_handler(regs: &mut Registers) {
    /* 割込み・例外要因 */
    let scause = Scause {};
    let e: usize = scause.read(scause::EXCEPTION) as usize;
    let i: usize = scause.read(scause::INTERRUPT) as usize;

    /* 割込み・例外ハンドラの呼出し */
    unsafe {
        match i {
            0 => match EXCEPTION_HANDLER[e] {
                Some(func) => func(regs),
                None => (),
            },
            1 => match INTERRUPT_HANDLER[e] {
                Some(func) => func(regs),
                None => (),
            },
            _ => (),
        };
    }
}

use crate::CPU;
#[no_mangle]
pub extern "C" fn get_cpuid() -> usize {
    unsafe {
        let scratch: &Scratch = transmute(CPU.csr.sscratch.get());
        if CPU.csr.sscratch.get() == 0 {
            0
        } else {
            scratch.cpu_id as usize
        }
    }
}

pub fn redirect_to_guest(regs: &mut Registers) {
    let hstatus = Hstatus {};
    let sstatus = Sstatus {};
    let vsstatus = Vsstatus {};
    let vsepc = Vsepc {};
    let sepc = Sepc {};
    let vscause = Vscause {};
    let scause = Scause {};
    let vstvec = Vstvec {};
    let stval = Stval {};
    let vstval = Vstval {};

    //1. vsstatus.SPP = sstatus.SPP
    match sstatus.read(sstatus::SPP) {
        1 => vsstatus.modify(vsstatus::SPP::SET),
        0 => vsstatus.modify(vsstatus::SPP::CLEAR),
        _ => (),
    }

    //2. vsstatus.SPIE = vsstatus.SIE
    let _s = vsstatus.read(vsstatus::SIE);
    match vsstatus.read(vsstatus::SIE) {
        1 => vsstatus.modify(vsstatus::SPIE::SET),
        0 => vsstatus.modify(vsstatus::SPIE::CLEAR),
        _ => (),
    }
    let _s2 = vsstatus.read(vsstatus::SIE);
    let _v = Vsie {}.get();
    // vsstatus.SIE = 0
    vsstatus.modify(vsstatus::SIE::CLEAR);

    // vscause = scause
    vscause.set(scause.get());
    // vstval = stval
    vstval.set(stval.get());
    // vsepc = sepc
    vsepc.set(sepc.get());

    //3. sepc = vstvec
    //sepc.set(vstvec.get());
    (*(regs)).epc = vstvec.get() as usize;

    //4. sstatus.SPP = 1
    sstatus.modify(sstatus::SPP::SET);

    //5. sret
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
