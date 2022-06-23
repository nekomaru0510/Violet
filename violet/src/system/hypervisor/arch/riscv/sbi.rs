//! SBI

use crate::CPU;

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
    Timer = 0x54494D45,
    Ipi = 0x735049,
    Rfence = 0x52464E43,
}

pub fn sbi_set_timer(stime_value: u64) -> (usize, usize) {
    let ext = Extension::SetTimer as i32;
    let fid = 0 as i32;
    let a0 = stime_value as usize;

    CPU.inst.do_ecall(ext, fid, a0, 0, 0, 0, 0, 0)
}
