//! RV64I CPU ドライバ

#![feature(naked_functions)]

pub mod boot;
use boot::_start_trap;

extern crate register;
use register::{cpu::RegisterReadWrite/*, register_bitfields*/};

extern crate alloc;
use alloc::string::String;

pub mod csr;
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
use csr::hvip::*;
use csr::hgatp::*;
use csr::sie::*;
use csr::sip::*;
use csr::scause::*;

pub struct Processor {
    pub index: u32,
    pub mtvec: Mtvec,
    pub mie: Mie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub mcause: Mcause,
}

////////////////////////////////
/* インスタンス化されたCPUのメソッド(他プロセッサからの処理要求) */
////////////////////////////////
impl Processor {
    pub fn new(index: u32) -> Self {
        Processor{index, mtvec: Mtvec {}, mie: Mie {}, mip: Mip {}, mepc: Mepc {}, mstatus: Mstatus {}, mcause: Mcause {}, }
    }
}

////////////////////////////////
/* 公開関数(自プロセッサの処理) */
////////////////////////////////
// そもそもオブジェクトで管理しているのは、リソース競合を気にしているため。
// しかし、自プロセッサであれば他コンテナで動作しているわけがないため、公開関数として実装

pub const PRV_MODE_U : u8 = 0x0;
pub const PRV_MODE_S : u8 = 0x1;
pub const PRV_MODE_M : u8 = 0x3;

#[no_mangle]
pub extern "C" fn jump_hyp_mode(next_addr: usize, arg1: usize, arg2: usize) -> usize {
    let mstatus = Mstatus{};

    /*
    if mstatus.read(mstatus::MPV) == 1 {
        return 1;
    }*/

    //let sstatus = Sstatus{};
    //sstatus.modify(sstatus::SPP::SET);

    mstatus.modify(mstatus::MPP::SUPERVISOR);
    mstatus.modify(mstatus::MPV::SET);

    unsafe {
        asm! ("
        .align 8
                csrw mepc, $0
                addi a0, $1, 0
                addi a1, $2, 0
                mret
        "
        :
        : "r"(next_addr), "r"(arg1), "r"(arg2) 
        :
        : "volatile");
    }

    return 0;
}

//pub fn jump_next_mode(mode: u8, next_addr: usize, arg1: usize, arg2: usize) {
//extern crate core;
//use core::intrinsics::transmute;
#[no_mangle]
pub extern "C" fn jump_next_mode(next_addr: usize, arg1: usize, arg2: usize) {
    let sstatus = Sstatus{};
    sstatus.modify(sstatus::SPP::SET);

    //let hstatus = Hstatus{};
    //hstatus.modify(hstatus::SPV::SET);
    
    let hedeleg = Hedeleg{};
    hedeleg.set((1 << 0) | (1 << 3) | (1 << 8) | (1 << 12) | (1 << 13) | (1 << 15));
    //hedeleg.set(0xffff_ffff ^ 0x400);

    let hideleg = Hideleg{};
    hideleg.set((1 << 10) | (1 << 6) | (1 << 2));
    //hideleg.set(0xffff_ffff);
    
    //let hvip = Hvip{};
    //hvip.set(0);

    let hgatp = Hgatp{};
    //hgatp.modify(hgatp::SV39X4::SET);
    //hgatp.modify(hgatp::MODE::BARE);
    hgatp.set(0);

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

pub fn setup_vector() {
    /*
    let mtvec = Mtvec{};
    mtvec.set(_start_trap as u64);
    */
    let stvec = Stvec{};
    stvec.set(_start_trap as u64);
}

pub fn get_cpuid() -> u64 {
    let mhartid = Mhartid{};
    mhartid.get()
}

pub fn enable_interrupt() {
/*
    let mie = Mie{};
    let mstatus = Mstatus{};
    
    mie.modify(mie::MSIE::SET);
    mie.modify(mie::MTIE::SET);
    mie.modify(mie::MEIE::SET);
    mstatus.modify(mstatus::MIE::SET);
*/
    let sie = Sie{};
    let sstatus = Sstatus{};
    
    sie.modify(sie::SSIE::SET);
    sie.modify(sie::STIE::SET);
    sie.modify(sie::SEIE::SET);
    sstatus.modify(sstatus::SIE::SET);
}

pub fn disable_interrupt() {
    let mie = Mie{};
    let mstatus = Mstatus{};

    mie.modify(mie::MSIE::CLEAR);
    mie.modify(mie::MTIE::CLEAR);
    mie.modify(mie::MEIE::CLEAR);
    mstatus.modify(mstatus::MIE::CLEAR);
}


////////////////////////////////
/* 関数(アセンブリから飛んでくる関数) */
////////////////////////////////

/* カーネルの起動処理 */
use crate::boot_init;

/* カーネル本体の割込みハンドラ */
use crate::interrupt_handler;
use crate::Context;

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
        : "memory");
        
        return (err, val);
    }
}

// CPU内 割込みハンドラ
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn get_context(sp :*mut RegisterStack) {

    let mut cont = Context::new();
    cont.regsize = 16;
    let ret = interrupt_handler(&mut cont);
    
    let hideleg = Hideleg{};
    hideleg.set(0xffff_ffff);

    unsafe {
        let scause = Scause{};
        let e = scause.read(scause::EXCEPTION);
        let i = scause.read(scause::INTERRUPT);

        /* VS-modeからのecallのみ、リダイレクトする */
        if e == 10 && i == 0 {
            let mut ext: i32 = (*(sp)).reg[15] as i32;
            let mut fid: i32 = (*(sp)).reg[14] as i32;
            let mut a0: usize = (*(sp)).reg[8];
            let mut a1: usize = (*(sp)).reg[9];
            let mut a2: usize = (*(sp)).reg[10];
            let mut a3: usize = (*(sp)).reg[11];
            let mut a4: usize = (*(sp)).reg[12];
            let mut a5: usize = (*(sp)).reg[13];

            if (*(sp)).reg[15] == 0x6 {
                ext = 0x52464E43;
                fid = 1;
                a0 = 1;
                a1 = 1;
                a2 = (*(sp)).reg[8];
                a3 = (*(sp)).reg[9];
            }
            let ret = do_ecall(ext, fid, a0, a1, a2, a3 , a4, a5);
            (*(sp)).reg[8] = ret.0;
            (*(sp)).reg[9] = ret.1;
        }
        else if i == 1 && e == 0x5 {
            let sie = Sie{};
            let sip = Sip{};

            sip.modify(sip::STIP::CLEAR);
            sie.modify(sie::VSTIE::SET);

            let hedeleg = Hedeleg{};
            hedeleg.set(0xffff_ffff ^ 0x400);
            let hideleg = Hideleg{};
            hideleg.set(0xffff_ffff);
        }
    }

    /* [todo fix] 割込みごとに(レジスタを読むために)毎回newするのはよろしくない気がするので、なるべくやめる */
    /*
    let cpu = Processor::new(0);
    cpu.mstatus.modify(mstatus::MPIE::SET); /* mstatusのMPIEには割込み元でのMIEビットが入る */
    cpu.mip.modify(mip::MTIP::CLEAR);       /* タイマ割込みがペンディングされてる？ためクリア(必要か？) */
    */

}

