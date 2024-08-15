//! Hypervisor guest address translation and protection Register (hgatp)

use crate::register;

register!(
    Hgatp,              /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x680",   /* Read Instruction */
    "csrw 0x680, {}",   /* Write Instruction */
    {                   /* Register Field */
        PPN       OFFSET(0)  NUMBITS(44) [],
        VMID      OFFSET(44)  NUMBITS(14) [],
        MODE      OFFSET(60)  NUMBITS(4) [
            BARE = 0,
            SV39X4 = 8,
            SV48X4 = 9,
            SV57X4 = 10 //Reserved
        ]
    }
);
