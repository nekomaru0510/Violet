//! Machine Cause Register(mcause)

use crate::register;

register!(
    Mcause,             /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x342",   /* Read Instruction */
    "csrw 0x342, $0",   /* Write Instruction */
    {                   /* Register Field */
        EXCEPTION_CODE  OFFSET(0)  NUMBITS(63) [
            USER_SOFTWARE = 0,
            SUPERVISOR_SOFTWARE = 1,
            MACHINE_SOFTWARE = 3,
            USER_TIMER = 4,
            SUPERVISOR_TIMER = 5,
            MACHINE_TIMER = 7,
            USER_EXTERNAL = 8,
            SUPERVISOR_EXTERNAL = 9,
            MACHINE_EXTERNAL = 11
        ],
        INTERRUPT       OFFSET(63)  NUMBITS(1) []
    }
);
