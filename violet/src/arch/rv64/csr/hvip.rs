//! Hypervisor virtual interrupt pending Register (hvip)

use crate::register;

register!(
    Hvip,               /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x645",   /* Read Instruction */
    "csrw 0x645, $0",   /* Write Instruction */
    {                   /* Register Field */
        VSSIP     OFFSET(2)  NUMBITS(1) [],
        VSTIP     OFFSET(6)  NUMBITS(1) [],
        VSEIP     OFFSET(10) NUMBITS(1) []
    }
);