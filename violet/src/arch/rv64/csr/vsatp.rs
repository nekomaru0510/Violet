//! Virtual Supervisor address translation and protection Register (vsatp)

use crate::register;

register!(
    Vsatp,              /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x280",   /* Read Instruction */
    "csrw 0x280, {}",   /* Write Instruction */
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
