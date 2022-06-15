//! RV64I CPU ドライバ

/* ドライバ用トレイト */
use crate::driver::traits::arch::riscv::Exception;
use crate::driver::traits::arch::riscv::Interrupt;
use crate::driver::traits::arch::riscv::PrivilegeMode;
use crate::driver::traits::arch::riscv::Registers;
use crate::driver::traits::arch::riscv::TraitRisvCpu;
use crate::driver::traits::cpu::TraitCpu;

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

extern crate register;
use register::cpu::RegisterReadWrite;

extern crate alloc;

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
    pub index: u32,          /* CPUのid */
    pub mode: PrivilegeMode, /* 動作モード */
    pub csr: Csr,            /* CSR [todo delete]*/
    pub inst: Rv64Inst,
    pub int: Rv64Int,
    pub exc: Rv64Exc,
    pub mmu: Rv64Mmu,
    pub hyp: Rv64Hyp,
}

use crate::print;
use crate::println;
////////////////////////////////
/* ハードウェア依存の機能の実装 */
///////////////////////////////
impl Rv64 {
    pub const fn new(index: u32) -> Self {
        Rv64 {
            index,
            mode: PrivilegeMode::ModeS,
            csr: Csr::new(),
            inst: Rv64Inst::new(),
            int: Rv64Int::new(),
            exc: Rv64Exc::new(),
            mmu: Rv64Mmu::new(),
            hyp: Rv64Hyp::new(),
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

    pub fn print_csr(&self) {
        println!("hstatus: {:x}", self.csr.hstatus.get());
        println!("hideleg: {:x}", self.csr.hideleg.get());
        println!("hedeleg: {:x}", self.csr.hedeleg.get());
        println!("hie: {:x}", self.csr.hie.get());
        println!("hip: {:x}", self.csr.hip.get());
        println!("hvip: {:x}", self.csr.hvip.get());

        println!("sstatus: {:x}", self.csr.sstatus.get());
        println!("sepc: {:x}", self.csr.sepc.get());
        println!("scause: {:x}", self.csr.scause.get());

        println!("vsstatus: {:x}", self.csr.vsstatus.get());
        println!("vsie: {:x}", self.csr.vsie.get());
        println!("vsip: {:x}", self.csr.vsip.get());
        println!("vsepc: {:x}", self.csr.vsepc.get());
        println!("vscause: {:x}", self.csr.vscause.get());
        println!("vstvec: {:x}", self.csr.vstvec.get());
    }

}

//////////////////////////////////////
/* (一般的な)CPUとして必要な機能の実装 */
//////////////////////////////////////
impl TraitCpu for Rv64 {
    fn enable_interrupt(&self) {
        self.int.enable_s();
    }

    fn disable_interrupt(&self) {
        self.int.disable_s();
    }
}

const NUM_OF_INTERRUPTS: usize = 32;
const NUM_OF_EXCEPTIONS: usize = 32;

pub static mut INTERRUPT_HANDLER: [Option<fn(regs: &mut Registers)>;
    NUM_OF_INTERRUPTS] = [None; NUM_OF_INTERRUPTS];
pub static mut EXCEPTION_HANDLER: [Option<fn(regs: &mut Registers)>;
    NUM_OF_EXCEPTIONS] = [None; NUM_OF_EXCEPTIONS];

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
use crate::boot_init;

// CPU初期化処理 ブート直後に実行される
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn setup_cpu() {
    boot_init();
    /*
    if get_cpuid() == 0 {
        /* BSPはboot_initへ */
        boot_init();
    } else {
        /* APは待ち */
        unsafe{asm!("wfi");}
    }*/
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
