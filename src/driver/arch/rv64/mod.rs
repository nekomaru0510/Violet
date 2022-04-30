//! RV64I CPU ドライバ

/* ドライバ用トレイト */
use crate::driver::traits::cpu::TraitCpu;

pub mod boot;
use boot::_start_trap;

extern crate register;
use register::{cpu::RegisterReadWrite/*, register_bitfields*/};

extern crate alloc;

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

pub const PRV_MODE_U : u8 = 0x0;
pub const PRV_MODE_S : u8 = 0x1;
pub const PRV_MODE_M : u8 = 0x3;

#[derive(Clone)]
pub struct Processor {
    pub index: u32,
    pub mode: u8,         /* 動作モード */
    pub mtvec: Mtvec,
    pub stvec: Stvec,
    pub mie: Mie,
    pub sie: Sie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub sstatus: Sstatus,
    pub mcause: Mcause,
}

////////////////////////////////
/* インスタンス化されたCPUのメソッド(他プロセッサからの処理要求) */
////////////////////////////////
impl Processor {
    pub fn new(index: u32) -> Self {
        Processor{index, mode:PRV_MODE_S, mtvec: Mtvec {}, stvec: Stvec {}, mie: Mie {}, sie: Sie {}, mip: Mip {}, mepc: Mepc {}, mstatus: Mstatus {}, sstatus: Sstatus {}, mcause: Mcause {}, }
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
                self.mtvec.set(addr as u64);
            },
            PRV_MODE_S => {
                self.stvec.set(addr as u64);
            },
            _ => {},
        }
    }

}

impl TraitCpu for Processor {
    fn enable_interrupt(&self) {
        match self.mode {
            PRV_MODE_M => {
                self.mie.modify(mie::MSIE::SET);
                self.mie.modify(mie::MTIE::SET);
                self.mie.modify(mie::MEIE::SET);
                self.mstatus.modify(mstatus::MIE::SET);
            },
            PRV_MODE_S => {
                self.sie.modify(sie::SSIE::SET);
                self.sie.modify(sie::STIE::SET);
                self.sie.modify(sie::SEIE::SET);
                self.sstatus.modify(sstatus::SIE::SET);
            },
            _ => {},
        }
    }

    fn disable_interrupt(&self) {
        match self.mode {
            PRV_MODE_M => {
                self.mie.modify(mie::MSIE::CLEAR);
                self.mie.modify(mie::MTIE::CLEAR);
                self.mie.modify(mie::MEIE::CLEAR);
                self.mstatus.modify(mstatus::MIE::CLEAR);
            },
            PRV_MODE_S => {
                self.sie.modify(sie::SSIE::CLEAR);
                self.sie.modify(sie::STIE::CLEAR);
                self.sie.modify(sie::SEIE::CLEAR);
                self.sstatus.modify(sstatus::SIE::CLEAR);
            },
            _ => {},
        }
    }
}

////////////////////////////////
/* 公開関数(自プロセッサの処理) */
////////////////////////////////
// そもそもオブジェクトで管理しているのは、リソース競合を気にしているため。
// しかし、自プロセッサであれば他コンテナで動作しているわけがないため、公開関数として実装


#[no_mangle]
pub extern "C" fn switch_hs_mode(next_addr: usize, arg1: usize, arg2: usize) -> usize {

    let mstatus = Mstatus{};
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

#[no_mangle]
pub extern "C" fn jump_guest_kernel(next_addr: usize, arg1: usize, arg2: usize) {
    let sstatus = Sstatus{};
    sstatus.modify(sstatus::SPP::SET);

    //let hstatus = Hstatus{};
    //hstatus.modify(hstatus::SPV::SET);
    
    let hedeleg = Hedeleg{};
    hedeleg.set((1 << 0) | (1 << 3) | (1 << 8) | (1 << 12) | (1 << 13) | (1 << 15));

    let hideleg = Hideleg{};
    hideleg.set((1 << 10) | (1 << 6) | (1 << 2));

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

pub fn get_cpuid() -> u64 {
    let mhartid = Mhartid{};
    mhartid.get()
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
        : );
        
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

            let ret = do_ecall(ext, fid, a0, a1, a2, a3 , a4, a5);
            (*(sp)).reg[8] = ret.0;
            (*(sp)).reg[9] = ret.1;
        }
    }

    /* [todo fix] 割込みごとに(レジスタを読むために)毎回newするのはよろしくない気がするので、なるべくやめる */
    /*
    let cpu = Processor::new(0);
    cpu.mstatus.modify(mstatus::MPIE::SET); /* mstatusのMPIEには割込み元でのMIEビットが入る */
    cpu.mip.modify(mip::MTIP::CLEAR);       /* タイマ割込みがペンディングされてる？ためクリア(必要か？) */
    */

}

