//! RV64I CPU ドライバ

pub mod boot;
pub mod csr;
pub mod extension;
pub mod instruction;
pub mod mmu;
pub mod regs;
pub mod sbi;
pub mod trap;
pub mod vscontext;

/*  */
use crate::environment::STACK_SIZE;
/* ドライバ用トレイト */
use crate::driver::traits::cpu::TraitCpu;
extern crate register;
use register::cpu::RegisterReadWrite;

use instruction::Instruction;
use mmu::Rv64Mmu;
use regs::Registers;
use trap::TrapVector;
use trap::_start_trap;
use trap::int::Interrupt;

extern crate core;
use core::intrinsics::transmute;

use csr::hstatus::*;
use csr::mtvec::Mtvec;
use csr::scause::Scause;
use csr::sepc::Sepc;
use csr::sscratch::Sscratch;
use csr::sstatus::*;
use csr::stval::Stval;
use csr::stvec::Stvec;
use csr::vscause::Vscause;
use csr::vsepc::Vsepc;
use csr::vsie::Vsie;
use csr::vsstatus::*;
use csr::vstval::Vstval;
use csr::vstvec::Vstvec;

pub struct Rv64 {
    pub scratch: Scratch,    /* scratchレジスタが指す構造体 */
    pub mode: PrivilegeMode, /* 動作モード */
    pub mmu: Rv64Mmu,
    trap: TrapVector,
}

#[derive(Copy, Clone)]
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
    status: CpuStatus,
}

impl Scratch {
    pub const fn new(cpu_id: u64) -> Self {
        Scratch {
            cpu_id,
            sp: 0x0,
            tmp0: 0x0,
            stack_size: STACK_SIZE,
            status: CpuStatus::STARTED,
        }
    }

    pub fn set_cpu_id(&mut self, cpu_id: u64) {
        self.cpu_id = cpu_id;
    }

    pub fn get_cpu_id(&self) -> u64 {
        self.cpu_id
    }
}

impl TraitCpu for Rv64 {
    type Registers = Registers;

    fn core_init(&self) {
        self.set_sscratch();
        self.set_default_vector();
        self.enable_interrupt();
    }

    fn wakeup(&self) {
        sbi::sbi_hart_start(self.scratch.cpu_id, boot::_start_ap as u64, 0xabcd);
    }

    fn sleep(&self) {
        sbi::sbi_hart_stop();
    }

    fn register_vector(&mut self, vecid: usize, func: fn(regs: &mut Self::Registers)) {
        self.trap.register_vector(vecid, func);
    }

    fn enable_interrupt(&self) {
        Interrupt::enable_s();
    }

    fn disable_interrupt(&self) {
        Interrupt::disable_s();
    }

    fn ipi(&self, core_id: usize) {
        let hart_mask: u64 = 0x01 << core_id;
        sbi::sbi_send_ipi(&hart_mask);
    }
}

////////////////////////////////
/* ハードウェア依存の機能の実装 */
///////////////////////////////
impl Rv64 {
    pub const fn new(id: u64) -> Self {
        Rv64 {
            mode: PrivilegeMode::ModeS,
            mmu: Rv64Mmu::new(),
            scratch: Scratch::new(id),
            trap: TrapVector::new(),
        }
    }

    pub fn set_sscratch(&self) {
        Sscratch.set(unsafe { transmute(&self.scratch) });
    }

    pub fn set_default_vector(&self) {
        self.set_vector(_start_trap as usize);
    }

    fn set_vector(&self, addr: usize) {
        match self.mode {
            PrivilegeMode::ModeM => {
                Mtvec.set(addr as u64);
            }
            PrivilegeMode::ModeS => {
                Stvec.set(addr as u64);
            }
            _ => {}
        }
    }

    pub fn switch_hs_mode() {
        /* 次の動作モードをHS-modeに */
        Self::set_next_mode(PrivilegeMode::ModeHS);
        /* 次の動作モードへ切替え */
        Instruction::sret(0, 0, 0);
    }

    pub fn set_next_mode(mode: PrivilegeMode) {
        match mode {
            PrivilegeMode::ModeS => {
                Sstatus.modify(sstatus::SPP::SET);
                Hstatus.modify(hstatus::SPV::CLEAR);
            }
            PrivilegeMode::ModeVS => {
                Sstatus.modify(sstatus::SPP::SET);
                Hstatus.modify(hstatus::SPV::SET);
                Hstatus.modify(hstatus::SPVP::SET);
            }
            PrivilegeMode::ModeHS => {
                Sstatus.modify(sstatus::SPP::SET);
                Hstatus.modify(hstatus::SPV::CLEAR);
            }
            _ => (),
        };
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

#[no_mangle]
pub extern "C" fn get_cpuid() -> usize {
    unsafe {
        let scratch: &Scratch = transmute(Sscratch.get());
        if Sscratch.get() == 0 {
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
