//! Hypervisor Counter-Enable Register (hcounteren)

use crate::register;

register!(
    Hcounteren,         /* Register Name */
    u32,                /* Register Size */
    "csrr $0, 0x606",   /* Read Instruction */
    "csrw 0x606, $0",   /* Write Instruction */
    {                   /* Register Field */
        CY         OFFSET(0)  NUMBITS(1) [],
        TM         OFFSET(1)  NUMBITS(1) [],
        IR         OFFSET(2)  NUMBITS(1) [],
        HPM3       OFFSET(3)  NUMBITS(1) [],
        HPM4       OFFSET(4)  NUMBITS(1) [],
        HPM5       OFFSET(5)  NUMBITS(1) [],
        HPM6       OFFSET(6)  NUMBITS(1) [],
        HPM7       OFFSET(7)  NUMBITS(1) [],
        HPM8       OFFSET(8)  NUMBITS(1) [],
        HPM9       OFFSET(9)  NUMBITS(1) [],
        HPM10      OFFSET(10) NUMBITS(1) [],
        HPM11      OFFSET(11) NUMBITS(1) [],
        HPM12      OFFSET(12) NUMBITS(1) [],
        HPM13      OFFSET(13) NUMBITS(1) [],
        HPM14      OFFSET(14) NUMBITS(1) [],
        HPM15      OFFSET(15) NUMBITS(1) [],
        HPM16      OFFSET(16) NUMBITS(1) [],
        HPM17      OFFSET(17) NUMBITS(1) [],
        HPM18      OFFSET(18) NUMBITS(1) [],
        HPM19      OFFSET(19) NUMBITS(1) [],
        HPM20      OFFSET(20) NUMBITS(1) [],
        HPM21      OFFSET(21) NUMBITS(1) [],
        HPM22      OFFSET(22) NUMBITS(1) [],
        HPM23      OFFSET(23) NUMBITS(1) [],
        HPM24      OFFSET(24) NUMBITS(1) [],
        HPM25      OFFSET(25) NUMBITS(1) [],
        HPM26      OFFSET(26) NUMBITS(1) [],
        HPM27      OFFSET(27) NUMBITS(1) [],
        HPM28      OFFSET(28) NUMBITS(1) [],
        HPM29      OFFSET(29) NUMBITS(1) [],
        HPM30      OFFSET(30) NUMBITS(1) [],
        HPM31      OFFSET(31) NUMBITS(1) []
    }
);
