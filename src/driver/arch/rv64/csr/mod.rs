//! CSR 

/* 0x100-0x5A8 */
pub mod sstatus;
pub mod sie;
pub mod stvec;
//pub mod scouteren;
//pub mod senvcfg;
//pub mod sscratch;
pub mod sepc;
pub mod scause;
pub mod stval;
pub mod sip;
//pub mod satp;
//pub mod scontext;

/* 0x600-0x615 */
pub mod hstatus;
pub mod hedeleg;
pub mod hideleg;
pub mod hie;
pub mod hcounteren;
pub mod hgeie;
//pub mod htval;
pub mod hip;
pub mod hvip;
//pub mod htinst;
//pub mod hgeip;
//pub mod hevcfg;
//pub mod hevcfgh;
pub mod hgatp;
//pub mod hcontext;
//pub mod htimedelta;
//pub mod htimedeltah;

/* 0x200-0x280 */
pub mod vsstatus;
pub mod vsie;
pub mod vstvec;
//pub mod vsscratch
pub mod vsepc;
pub mod vscause;
pub mod vstval;
pub mod vsip;
//pub mod vsatp;

/* 0xF11- */
pub mod mtvec;
pub mod mie;
pub mod mip;
pub mod mepc;
pub mod mstatus;
pub mod mcause;
pub mod mhartid;

use sstatus::*;
use sie::*;
use sip::*;
use scause::*;
use sepc::*;
use stval::*;
use stvec::*;

use hstatus::*;
use hedeleg::*;
use hideleg::*;
use hcounteren::*;
use hip::*;
use hvip::*;
use hie::*;
use hgatp::*;
use hgeie::*;

use vsstatus::*;
use vsie::*;
use vstvec::*;
use vsepc::*;
use vscause::*;
use vstval::*;
use vsip::*;

use mtvec::*;
use mie::*;
use mip::*;
use mepc::*;
use mstatus::*;
use mcause::*;
use mhartid::*;

#[derive(Clone)]
pub struct Csr {
    pub sstatus: Sstatus,
    pub sie: Sie,
    pub stvec: Stvec,
    pub sepc: Sepc,
    pub scause: Scause,
    pub stval: Stval,
    pub sip: Sip,

    pub hstatus: Hstatus,
    pub hedeleg: Hedeleg,
    pub hideleg: Hideleg,
    pub hie: Hie,
    pub hcounteren: Hcounteren,
    pub hgeie: Hgeie,
    pub hip: Hip,
    pub hvip: Hvip,
    pub hgatp: Hgatp,
    
    pub vsstatus: Vsstatus,
    pub vsie: Vsie,
    pub vstvec: Vstvec,
    pub vsepc: Vsepc,
    pub vscause: Vscause,
    pub vstval: Vstval,
    pub vsip: Vsip,
    
    pub mtvec: Mtvec,
    pub mie: Mie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub mcause: Mcause,
    pub mhartid: Mhartid,
}

impl Csr {
    pub fn new() -> Csr {
        Csr {
            sstatus: Sstatus{},
            sie: Sie{},
            stvec: Stvec{},
            sepc: Sepc{},
            scause: Scause{},
            stval: Stval{},
            sip: Sip{},
        
            hstatus: Hstatus{},
            hedeleg: Hedeleg{},
            hideleg: Hideleg{},
            hie: Hie{},
            hcounteren: Hcounteren{},
            hgeie: Hgeie{},
            hip: Hip{},
            hvip: Hvip{},
            hgatp: Hgatp{},
        
            vsstatus: Vsstatus{},
            vsie: Vsie{},
            vstvec: Vstvec{},
            vsepc: Vsepc{},
            vscause: Vscause{},
            vstval: Vstval{},
            vsip: Vsip{},
        
            mtvec: Mtvec{},
            mie: Mie{},
            mip: Mip{},
            mepc: Mepc{},
            mstatus: Mstatus{},
            mcause: Mcause{},
            mhartid: Mhartid{},
        }
    }
}