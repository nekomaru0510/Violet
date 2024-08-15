//! Supervisor address translation and protection Register (satp)

use crate::register;

register!(
    Satp,               /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x180",   /* Read Instruction */
    "csrw 0x180, {}",   /* Write Instruction */
    {                   /* Register Field */
        PPN       OFFSET(0)  NUMBITS(44) [],
        ASID      OFFSET(44)  NUMBITS(16) [],
        MODE      OFFSET(60)  NUMBITS(4) [
            BARE = 0,
            SV39X4 = 8,
            SV48X4 = 9,
            SV57X4 = 10 //Reserved
        ]
    }
);
