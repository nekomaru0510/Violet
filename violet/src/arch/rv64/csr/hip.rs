//! Hypervisor interrupt-pending register(hip)

use crate::register;

register!(
    Hip,                /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x644",   /* Read Instruction */
    "csrw 0x644, $0",   /* Write Instruction */
    {                   /* Register Field */
        VSSIP     OFFSET(2)  NUMBITS(1) [],
        VSTIP     OFFSET(6)  NUMBITS(1) [],
        VSEIP     OFFSET(10) NUMBITS(1) [],
        SGEIE     OFFSET(12) NUMBITS(1) []
    }
);
