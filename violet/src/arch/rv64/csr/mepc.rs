//! Machine Exception Program Counter(mepc)

use crate::register;

register!(
    Mepc,               /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x341",   /* Read Instruction */
    "csrw 0x341, {}",   /* Write Instruction */
    {                   /* Register Field */
        MEPC       OFFSET(0)  NUMBITS(32) []
    }
);
