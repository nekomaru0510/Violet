//! RV64I CPU ドライバ

/*  */
use crate::environment::STACK_SIZE;

/* ドライバ用トレイト */
use crate::driver::traits::cpu::TraitCpu;

pub mod regs;
use regs::Registers;

pub mod boot;

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

pub mod vscontext;

pub mod trap;
use trap::TrapHandler;
use trap::_start_trap;

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

//#[derive(Clone)]
pub struct Rv64 {
    pub scratch: Scratch,    /* scratchレジスタが指す構造体 */
    pub id: u64,             /* CPUのid */
    pub status: CpuStatus,   /* 状態 */
    pub mode: PrivilegeMode, /* 動作モード */
    pub csr: Csr,            /* CSR [todo delete]*/
    pub inst: Rv64Inst,
    pub int: Rv64Int,
    pub exc: Rv64Exc,
    pub mmu: Rv64Mmu,
    pub hyp: Rv64Hyp,
    trap: TrapHandler,
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
            trap: TrapHandler::new(),
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

    fn set_vector(&self, addr: usize) {
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

    pub fn register_interrupt(&mut self, int_num: Interrupt, func: fn(regs: &mut Registers)) {
        self.trap.register_interrupt(int_num, func);
    }

    pub fn register_exception(&mut self, exc_num: Exception, func: fn(regs: &mut Registers)) {
        self.trap.register_exception(exc_num, func);
    }

    pub fn switch_hs_mode(&self) {
        /* 次の動作モードをHS-modeに */
        self.set_next_mode(PrivilegeMode::ModeHS);
        /* 次の動作モードへ切替え */
        self.inst.jump_by_sret(0, 0, 0);
    }

    pub fn set_next_mode(&self, mode: PrivilegeMode) {
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

    fn ipi(&self, core_id: usize) {
        let hart_mask: u64 = 0x01 << core_id;
        sbi::sbi_send_ipi(&hart_mask);
    }
}

/* カーネルの起動処理 */
use crate::kernel::boot_init;

// CPU初期化処理 ブート直後に実行される
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn setup_cpu(cpu_id: usize) {
    boot_init(cpu_id);
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

#[test_case]
fn test_rv64() -> Result<(), &'static str> {
    /*
        cpu!()
        cpu().

        /*  */
        cpu().register_interrupt(
            Interrupt::SupervisorExternalInterrupt,
            do_supervisor_external_interrupt,
        );

        /* トレイトに指定されてる機能 割込みを有効化 */
        cpu().enable_int(
            Interrupt::SupervisorTimerInterrupt | Interrupt::SupervisorExternalInterrupt,
        );

        /* トレイトに指定されていない機能(↓の機能はトレイトに入れてもいいかも) */
        cpu().inst.fetch(addr);
        Rv64::get_cpuid()
        rv64::Instruction::fetch(addr);
        Instruction::fetch(addr);
        Rv64::Csr::stvec::set(addr);

        /* CPUの拡張機能追加関連は、トレイトでまとめてもよい */
        /* register(id: usize, obj: T) */
        /* as_ref, as_mutみたいな感じで。hashmapとか使える？ */
        /* 命令の追加 */
        cpu().inst.insert(id, Instruction::new(format, opcode, funct));
        cpu().inst.get(id); /* 命令の取得(Vecみたいにアクセスしたい) */
        cpu().inst.analyse(inst); /* 命令の判定 */
        cpu().inst[id].call();    /* 命令の呼び出し */

        /* レジスタ(CSR)の追加 */
        cpu().csr.register(id, Csr::new());
        cpu().csr(stvec).read(); /* read/write */

        /* 例外・割込みの追加 */
        cpu().int.register(id);
        cpu().exc.register(id);
    */
    /* MMU機能追加(難しい。後で) */
    Ok(())
}
