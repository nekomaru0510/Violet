//! VSSCRATCH register

use crate::regfield;
use crate::regfunc;

regfield! (
    Vsscratch,  /* Register Name */
    u64,        /* Register Size */
    {           /* Register Field */
        VSSIE OFFSET(2) NUMBITS(1) [],
        VSTIE OFFSET(6) NUMBITS(1) [],
        VSEIE OFFSET(10) NUMBITS(1) [],
        SGEIE OFFSET(12) NUMBITS(1) [],
        MPP   OFFSET(16) NUMBITS(2) [
            USER = 0,
            SUPERVISOR = 1,
            RESERVED = 2,
            MACHINE = 3
        ]
    }
);

regfunc!(
    Vsscratch,          /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x240",   /* Read */
    "csrw 0x240, $0"    /* Write */
);