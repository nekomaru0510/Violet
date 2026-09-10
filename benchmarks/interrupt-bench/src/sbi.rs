//! SBI

use core::intrinsics::transmute;
use core::arch::asm;

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

pub fn sbi_set_timer(stime_value: u64) -> (usize, usize) {
    let ext = Extension::SetTimer as i32;
    let fid = 0 as i32;
    let a0 = stime_value as usize;

    ecall(ext, fid, a0, 0, 0, 0, 0, 0)
}

fn ecall(
    ext: i32,
    fid: i32,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> (usize, usize) {
    // println!("ecall: ext: {:x}, fid: {:x}, arg0: {:x}, arg1: {:x}, arg2: {:x}, arg3: {:x}, arg4: {:x}, arg5: {:x}", ext, fid, arg0, arg1, arg2, arg3, arg4, arg5);
    let mut val: usize;
    let mut err: usize;
    unsafe {
        asm! ("
        .align 8
                ecall
        ",
        inout("a0") arg0 => err,
        inout("a1") arg1 => val,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
        in("a6") fid,
        in("a7") ext,
    );

        return (err, val);
    }
}