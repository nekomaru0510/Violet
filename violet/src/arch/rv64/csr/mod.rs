//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! CSR

/* 0x100-0x5A8 */
pub mod sie;
pub mod sstatus;
pub mod stvec;
//pub mod scouteren;
//pub mod senvcfg;
pub mod satp;
pub mod scause;
pub mod sepc;
pub mod sip;
pub mod sscratch;
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
pub mod vsscratch;
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