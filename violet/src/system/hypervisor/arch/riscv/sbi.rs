//! SBI

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