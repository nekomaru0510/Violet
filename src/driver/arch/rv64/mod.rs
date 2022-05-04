//! RV64I CPU ドライバ

/* ドライバ用トレイト */
use crate::driver::traits::cpu::TraitCpu;
use crate::driver::traits::arch::riscv::TraitRisvCpu;
use crate::driver::traits::arch::riscv::Registers;
use crate::driver::traits::arch::riscv::PrivilegeMode;
use crate::driver::traits::arch::riscv::PagingMode;
use crate::driver::traits::arch::riscv::Exception;
use crate::driver::traits::arch::riscv::Interrupt;

pub mod boot;
use boot::_start_trap;

extern crate register;
use register::{cpu::RegisterReadWrite/*, register_bitfields*/};

extern crate alloc;

pub mod csr;
use csr::Csr;
use csr::mtvec::*;
use csr::stvec::*;
use csr::mie::*;
use csr::mip::*;
use csr::mepc::*;
use csr::mstatus::*;
use csr::mcause::*;
use csr::mhartid::*;
use csr::sstatus::*;
use csr::hstatus::*;
use csr::hedeleg::*;
use csr::hideleg::*;
use csr::hcounteren::*;
use csr::hvip::*;
use csr::hie::*;
use csr::hgatp::*;
use csr::hgeie::*;
use csr::sie::*;
use csr::sip::*;
use csr::scause::*;
use csr::sepc::*;
use csr::stval::*;
use csr::vsstatus::*;
use csr::vsepc::*;
use csr::vstvec::*;
use csr::vscause::*;
use csr::vstval::*;
use csr::vsie::*;

pub const PRV_MODE_U : u8 = 0x0;
pub const PRV_MODE_S : u8 = 0x1;
pub const PRV_MODE_M : u8 = 0x3;

#[derive(Clone)]
pub struct Rv64 {
    pub index: u32,     /* CPUのid */
    pub mode: u8,       /* 動作モード */
    pub csr: Csr,           /* CSR */
}

use crate::println;
use crate::print;
////////////////////////////////
/* ハードウェア依存の機能の実装 */
///////////////////////////////
impl Rv64 {
    pub fn new(index: u32) -> Self {
        Rv64{index, mode:PRV_MODE_S, csr: Csr::new(), }
    }
    // 現在の動作モードを返す
    pub fn current_privilege_mode(self) -> u8 {
        self.mode
    }

    pub fn set_default_vector(&self) {
        self.set_vector(_start_trap as usize);
    }

    pub fn set_vector(&self, addr: usize) {
        match self.mode {
            PRV_MODE_M => {
                self.csr.mtvec.set(addr as u64);
            },
            PRV_MODE_S => {
                self.csr.stvec.set(addr as u64);
            },
            _ => {},
        }
    }

    pub fn print_csr(&self) 
    {
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
        match self.mode {
            PRV_MODE_M => {
                self.csr.mstatus.modify(mstatus::MIE::SET);
            },
            PRV_MODE_S => {
                self.csr.sstatus.modify(sstatus::SIE::SET);
            },
            _ => {},
        }
    }

    fn disable_interrupt(&self) {
        match self.mode {
            PRV_MODE_M => {
                self.csr.mstatus.modify(mstatus::MIE::CLEAR);
            },
            PRV_MODE_S => {
                self.csr.sstatus.modify(sstatus::SIE::CLEAR);
            },
            _ => {},
        }
    }
}

const NUM_OF_INTERRUPTS: usize = 32;
const NUM_OF_EXCEPTIONS: usize = 32;

pub static mut INTERRUPT_HANDLER: [Option<fn(int_num: usize, regs: &mut Registers)>; NUM_OF_INTERRUPTS] = [None; NUM_OF_INTERRUPTS];
pub static mut EXCEPTION_HANDLER: [Option<fn(exc_num: usize, regs: &mut Registers)>; NUM_OF_EXCEPTIONS] = [None; NUM_OF_EXCEPTIONS];

////////////////////////////////
/* アーキテクチャ依存機能の実装 */
///////////////////////////////
impl TraitRisvCpu for Rv64 {
    fn register_interrupt(&self, int_num: usize, func: fn(int_num: usize, regs: &mut Registers)) {
        if ( int_num >= NUM_OF_INTERRUPTS ) {
            return ();
        }
        unsafe {
            INTERRUPT_HANDLER[int_num] = Some(func);
        }
    }

    fn register_exception(&self, exc_num: usize, func: fn(exc_num: usize, regs: &mut Registers)) {
        if ( exc_num >= NUM_OF_EXCEPTIONS ) {
            return ();
        }
        unsafe {
            EXCEPTION_HANDLER[exc_num] = Some(func);
        }
    }
    
    fn switch_hs_mode(&self) {
        /* 次の動作モードをHS-modeに */
        self.set_next_mode(PrivilegeMode::ModeHS);
        /* 次の動作モードへ切替え */
        jump_by_sret(0, 0, 0);
    }
    
    fn set_next_mode(&self, mode: PrivilegeMode) {
        match mode {
            PrivilegeMode::ModeS => {
                self.csr.sstatus.modify(sstatus::SPP::SET);
                self.csr.hstatus.modify(hstatus::SPV::CLEAR);  
            },
            PrivilegeMode::ModeVS => {
                self.csr.sstatus.modify(sstatus::SPP::SET);
                self.csr.hstatus.modify(hstatus::SPV::SET);  
            },
            PrivilegeMode::ModeHS => {
                self.csr.sstatus.modify(sstatus::SPP::SET);
                self.csr.hstatus.modify(hstatus::SPV::CLEAR);
            },
            _ => ()
        };
    }

    fn enable_interrupt_mask(&self, int_mask: usize) {
        let sint_mask = 0x222 & int_mask;  // sieの有効ビットでマスク
        let scurrent = self.csr.sie.get();
        self.csr.sie.set(scurrent | sint_mask as u64);

        let hint_mask = 0x1444 & int_mask; // hieの有効ビットでマスク
        let hcurrent = self.csr.hie.get();
        self.csr.hie.set(hcurrent | hint_mask as u64);
    }

    fn disable_interrupt_mask(&self, int_mask: usize) {
        let sint_mask = 0x222 & int_mask;  // sieの有効ビットでマスク
        let scurrent = self.csr.sie.get();
        self.csr.sie.set(scurrent & !(sint_mask as u64));

        let hint_mask = 0x1444 & int_mask; // hieの有効ビットでマスク
        let hcurrent = self.csr.hie.get();
        self.csr.hie.set(hcurrent & !(hint_mask as u64));
    }

    fn enable_external_interrupt_mask(&self, int_mask: usize) {
        let current = self.csr.hgeie.get();
        self.csr.hgeie.set(current | int_mask as u64);
    }

    fn disable_external_interrupt_mask(&self, int_mask: usize) {
        let current = self.csr.hgeie.get();
        self.csr.hgeie.set(current & !(int_mask as u64));
    }

    fn enable_interrupt_delegation_mask(&self, int_mask: usize) {
        let current = self.csr.hideleg.get();
        self.csr.hideleg.set(current | int_mask as u64);
    }

    fn disable_interrupt_delegation_mask(&self, int_mask: usize) {
        let current = self.csr.hideleg.get();
        self.csr.hideleg.set(current & !(int_mask as u64));
    }

    fn enable_exception_delegation_mask(&self, exc_mask: usize) {
        let current = self.csr.hedeleg.get();
        self.csr.hedeleg.set(current | exc_mask as u64);
    }

    fn disable_exception_delegation_mask(&self, exc_mask: usize) {
        let current = self.csr.hedeleg.get();
        self.csr.hedeleg.set(current & !(exc_mask as u64));
    }

    fn flush_vsmode_interrupt(&self) {
        self.csr.hvip.set(0);
    }

    fn assert_vsmode_interrupt(&self, int_mask:usize) {
        self.csr.hvip.set(int_mask as u64);   
        self.csr.hip.set((int_mask >> 1) as u32);
    }

    fn enable_vsmode_counter_access(&self, counter_mask:usize) {
        let current = self.csr.hcounteren.get();
        self.csr.hcounteren.set(current | counter_mask as u32);
    }
    
    fn disable_vsmode_counter_access(&self, counter_mask:usize) {
        let current = self.csr.hcounteren.get();
        self.csr.hcounteren.set(current & !(counter_mask as u32));
    } 

    fn set_paging_mode(&self, mode: PagingMode) {
        match mode {
            PagingMode::Bare => {self.csr.hgatp.modify(hgatp::MODE::BARE);},
            PagingMode::Sv39x4 => {self.csr.hgatp.modify(hgatp::MODE::SV39X4);},
            PagingMode::Sv48x4 => {self.csr.hgatp.modify(hgatp::MODE::SV48X4);},
            PagingMode::Sv57x4 => {self.csr.hgatp.modify(hgatp::MODE::SV57X4);},
        };
    }

}

pub fn jump_by_sret(next_addr: usize, arg1: usize, arg2: usize) {
    if next_addr == 0 {
        unsafe {
            asm! ("
            .align 8
                    la  a0, next
                    csrw sepc, a0
                    sret
            next:
                    nop
            "
            :
            :
            :
            : "volatile");
        }
    }
    else {
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

/*
#[no_mangle]
pub extern "C" fn _jump_guest_kernel(next_addr: usize, arg1: usize, arg2: usize) 
{
    
    let sstatus = Sstatus{};
    let hstatus = Hstatus{};
    
    // sret後に、VS-modeに移行させる
    sstatus.modify(sstatus::SPP::SET);
    hstatus.modify(hstatus::SPV::SET);    

    //VGEIN
    hstatus.set(hstatus.get() | 0x20 << 12);
    //hstatus.set(0x01 << 12);
    
    // 割込みを無効化
    let hie = Hie{};
    let sie = Sie{};
    let hgeie = Hgeie{};
    
    hie.set(0xffff_ffff);
    sie.set(0);
    hgeie.set(0xffff_ffff_ffff_ffff);

    let hedeleg = Hedeleg{};
    hedeleg.set((1 << 0) | (1 << 3) | (1 << 8) | (1 << 12) | (1 << 13) | (1 << 15));

    let hideleg = Hideleg{};
    hideleg.set((1 << 10) | (1 << 6) | (1 << 2));
    
    // VS-mode 割込みのフラッシュ
    let hvip = Hvip{};
    hvip.set(0);

    // 仮想記憶管理
    let hgatp = Hgatp{};
    hgatp.set(0);

    // 
    let hcounteren = Hcounteren{};
    hcounteren.set(0xffff_ffff);

    //Self::jump_by_sret(next_addr, arg1, arg2);
    // VS-modeへジャンプ
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

pub fn get_cpuid() -> u64 {
    let mhartid = Mhartid{};
    mhartid.get()
}
*/

////////////////////////////////
/* 関数(アセンブリから飛んでくる関数) */
////////////////////////////////

/* カーネルの起動処理 */
use crate::boot_init;

/* カーネル本体の割込みハンドラ */
//use crate::Context;

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RegisterStack {
    pub reg: [usize; 32], 
}

pub extern "C" fn do_ecall(ext: i32, fid: i32, mut arg0: usize, mut arg1: usize, arg2 :usize,
                            arg3 :usize, arg4 :usize, arg5 :usize,) -> (usize, usize){
    unsafe{
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


pub fn do_ecall_from_vsmode(exc_num: usize, regs: &mut Registers)
{
    let mut ext: i32 = (*(regs)).gp[15] as i32;
    let mut fid: i32 = (*(regs)).gp[14] as i32;
    let mut a0: usize = (*(regs)).gp[8];
    let mut a1: usize = (*(regs)).gp[9];
    let mut a2: usize = (*(regs)).gp[10];
    let mut a3: usize = (*(regs)).gp[11];
    let mut a4: usize = (*(regs)).gp[12];
    let mut a5: usize = (*(regs)).gp[13];

    if (ext == 6) {
        ext = 0x52464E43;
        fid = 6;
        a2 = a1;
        a3 = a2;
        a0 = 1;
        a1 = 0;
    }

    let ret = do_ecall(ext, fid, a0, a1, a2, a3 , a4, a5);

    (*(regs)).gp[8] = ret.0;
    (*(regs)).gp[9] = ret.1;

    (*(regs)).epc = (*(regs)).epc + 4;
}

// CPU内 割込みハンドラ
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn get_context(regs :&mut Registers) 
{
    /* 割込み元のモードを確認 */
    let hstatus = Hstatus{};
    let sstatus = Sstatus{};
    let _spv = hstatus.read(hstatus::SPV);
    let _spp = sstatus.read(sstatus::SPP);

    /* 割込み要因 */
    let scause = Scause{};
    let e: usize = scause.read(scause::EXCEPTION) as usize;
    let i: usize = scause.read(scause::INTERRUPT) as usize;

    /* 割込みハンドラの呼出し */
    unsafe {
        match i {
            0 => {
                match EXCEPTION_HANDLER[e] {
                    Some(func) => func(e, regs),
                    None => (),
                }
            },
            1 => {
                match INTERRUPT_HANDLER[e] {
                    Some(func) => func(e, regs),
                    None => (),
                }
            },
            _ => ()
        };
    }

}

pub fn redirect_to_guest(regs: &mut Registers) {
    let hstatus = Hstatus{};
    let sstatus = Sstatus{};
    let vsstatus = Vsstatus{};
    let vsepc = Vsepc{};
    let sepc = Sepc{};
    let vscause = Vscause{};
    let scause = Scause{};
    let vstvec = Vstvec{};
    let stval = Stval{};
    let vstval = Vstval{};
    
    //1. vsstatus.SPP = sstatus.SPP
    match sstatus.read(sstatus::SPP) {
        1 => vsstatus.modify(vsstatus::SPP::SET),
        0 => vsstatus.modify(vsstatus::SPP::CLEAR),
        _ => ()
    }
        
    //2. vsstatus.SPIE = vsstatus.SIE
    let _s = vsstatus.read(vsstatus::SIE);
    match vsstatus.read(vsstatus::SIE) {
        1 => vsstatus.modify(vsstatus::SPIE::SET),
        0 => vsstatus.modify(vsstatus::SPIE::CLEAR),
        _ => ()
    }
    let _s2 = vsstatus.read(vsstatus::SIE);
    let _v = Vsie{}.get();
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



