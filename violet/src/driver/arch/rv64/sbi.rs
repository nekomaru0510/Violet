//! SBI

use crate::CPU;

use core::intrinsics::transmute;

pub enum Extension {
    SetTimer = 0x00,
    ConsolePutchar,
    ConsoleGetchar,
    ClearIpi,
    SendIpi,
    RemoteFenceI,
    RemoteSfenceVma,
    RemoteSfenceVmaWithAsid,
    SystemShutdown,
    Base = 0x10,
    HartStateManagement = 0x48534D,
    Timer = 0x54494D45,
    Ipi = 0x735049,
    Rfence = 0x52464E43,
    SystemReset = 0x53525354,
    Unknown,
}

impl Extension {
    pub fn from_ext(ext: i32) -> Self {
        match ext {
            0x00000000 => Extension::SetTimer,
            0x00000001 => Extension::ConsolePutchar,
            0x00000002 => Extension::ConsoleGetchar,
            0x00000003 => Extension::ClearIpi,
            0x00000004 => Extension::SendIpi,
            0x00000005 => Extension::RemoteFenceI,
            0x00000006 => Extension::RemoteSfenceVma,
            0x00000007 => Extension::RemoteSfenceVmaWithAsid,
            0x00000008 => Extension::SystemShutdown,
            0x00000010 => Extension::Base,
            0x0048534D => Extension::HartStateManagement,
            0x00735049 => Extension::Ipi,
            0x54494D45 => Extension::Timer,
            0x52464E43 => Extension::Rfence,
            0x53525354 => Extension::Base,
            _ => Extension::Unknown,
        }
    }
}

pub fn sbi_set_timer(stime_value: u64) -> (usize, usize) {
    let ext = Extension::SetTimer as i32;
    let fid = 0 as i32;
    let a0 = stime_value as usize;

    CPU.inst.do_ecall(ext, fid, a0, 0, 0, 0, 0, 0)
}

pub fn sbi_send_ipi(hart_mask: &u64) -> (usize, usize) {
    let ext = Extension::SendIpi as i32;
    let fid = 0 as i32;
    let a0: usize = unsafe { transmute(hart_mask) };

    CPU.inst.do_ecall(ext, fid, a0, 0, 0, 0, 0, 0)
}

pub fn sbi_hart_start(hartid: u64, start_addr: u64, opaque: u64) -> (usize, usize) {
    let ext = Extension::HartStateManagement as i32;
    let fid = 0 as i32;
    let a0 = hartid as usize;
    let a1 = start_addr as usize;
    let a2 = opaque as usize;

    CPU.inst.do_ecall(ext, fid, a0, a1, a2, 0, 0, 0)
}

pub fn sbi_hart_stop() -> (usize, usize) {
    let ext = Extension::HartStateManagement as i32;
    let fid = 1 as i32;

    CPU.inst.do_ecall(ext, fid, 0, 0, 0, 0, 0, 0)
}

pub fn sbi_hart_getstatus(hartid: u64) -> (usize, usize) {
    let ext = Extension::HartStateManagement as i32;
    let fid = 2 as i32;
    let a0 = hartid as usize;

    CPU.inst.do_ecall(ext, fid, a0, 0, 0, 0, 0, 0)
}

pub fn sbi_hart_suspend(suspend_type: u32, resume_addr: u64, opaque: u64) -> (usize, usize) {
    let ext = Extension::HartStateManagement as i32;
    let fid = 3 as i32;
    let a0 = suspend_type as usize;
    let a1 = resume_addr as usize;
    let a2 = opaque as usize;

    CPU.inst.do_ecall(ext, fid, a0, a1, a2, 0, 0, 0)
}
