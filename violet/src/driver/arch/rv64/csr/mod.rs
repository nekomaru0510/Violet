//! CSR

/* 0x100-0x5A8 */
pub mod sie;
pub mod sstatus;
pub mod stvec;
//pub mod scouteren;
//pub mod senvcfg;
pub mod sscratch;
pub mod satp;
pub mod scause;
pub mod sepc;
pub mod sip;
pub mod stval;
//pub mod scontext;

/* 0x600-0x615 */
pub mod hcounteren;
pub mod hedeleg;
pub mod hgeie;
pub mod hideleg;
pub mod hie;
pub mod hip;
pub mod hstatus;
pub mod htval;
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
pub mod vsie;
pub mod vsstatus;
pub mod vstvec;
//pub mod vsscratch
pub mod vsatp;
pub mod vscause;
pub mod vsepc;
pub mod vsip;
pub mod vstval;

/* 0xF11- */
pub mod mcause;
pub mod mepc;
pub mod mhartid;
pub mod mie;
pub mod mip;
pub mod mstatus;
pub mod mtvec;

use satp::*;
use scause::*;
use sepc::*;
use sie::*;
use sip::*;
use sstatus::*;
use stval::*;
use stvec::*;
use sscratch::*;

use hcounteren::*;
use hedeleg::*;
use hgatp::*;
use hgeie::*;
use hideleg::*;
use hie::*;
use hip::*;
use hstatus::*;
use hvip::*;

use vsatp::*;
use vscause::*;
use vsepc::*;
use vsie::*;
use vsip::*;
use vsstatus::*;
use vstval::*;
use vstvec::*;

use mcause::*;
use mepc::*;
use mhartid::*;
use mie::*;
use mip::*;
use mstatus::*;
use mtvec::*;

#[derive(Clone)]
pub struct Csr {
    pub sstatus: Sstatus,
    pub sie: Sie,
    pub stvec: Stvec,
    pub sscratch: Sscratch,
    pub sepc: Sepc,
    pub scause: Scause,
    pub stval: Stval,
    pub sip: Sip,
    pub satp: Satp,

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
    pub vsatp: Vsatp,

    pub mtvec: Mtvec,
    pub mie: Mie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub mcause: Mcause,
    pub mhartid: Mhartid,
}

impl Csr {
    pub const fn new() -> Csr {
        Csr {
            sstatus: Sstatus {},
            sie: Sie {},
            stvec: Stvec {},
            sscratch: Sscratch {},
            sepc: Sepc {},
            scause: Scause {},
            stval: Stval {},
            sip: Sip {},
            satp: Satp {},

            hstatus: Hstatus {},
            hedeleg: Hedeleg {},
            hideleg: Hideleg {},
            hie: Hie {},
            hcounteren: Hcounteren {},
            hgeie: Hgeie {},
            hip: Hip {},
            hvip: Hvip {},
            hgatp: Hgatp {},

            vsstatus: Vsstatus {},
            vsie: Vsie {},
            vstvec: Vstvec {},
            vsepc: Vsepc {},
            vscause: Vscause {},
            vstval: Vstval {},
            vsip: Vsip {},
            vsatp: Vsatp {},

            mtvec: Mtvec {},
            mie: Mie {},
            mip: Mip {},
            mepc: Mepc {},
            mstatus: Mstatus {},
            mcause: Mcause {},
            mhartid: Mhartid {},
        }
    }
}
