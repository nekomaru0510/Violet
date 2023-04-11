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
    let a0: usize = unsafe {transmute(hart_mask)};

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
