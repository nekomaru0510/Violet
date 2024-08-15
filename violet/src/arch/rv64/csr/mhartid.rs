//! Machine Hart ID(mhartid)

use crate::register;

register!(
    Mhartid,            /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0xF14",   /* Read Instruction */
    "csrw 0xF14, {}",   /* Write Instruction */
    {                   /* Register Field */
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    }
);
