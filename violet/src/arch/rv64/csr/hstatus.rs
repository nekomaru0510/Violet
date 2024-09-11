//! Hypervisor Status Register (hstatus)

use crate::register;

register!(
    Hstatus,            /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x600",   /* Read Instruction */
    "csrw 0x600, {}",   /* Write Instruction */
    {                   /* Register Field */
        VSBE       OFFSET(5)  NUMBITS(1) [],    // Endian setting
        GVA        OFFSET(6)  NUMBITS(1) [],    //
        SPV        OFFSET(7)  NUMBITS(1) [],    // Virtualization state before trap
        SPVP       OFFSET(8)  NUMBITS(1) [],    // Privilege state before trap
        HU         OFFSET(9)  NUMBITS(1) [],    // If value is 1, HU-mode can use HS-specific instructions
        VGEIN      OFFSET(12)  NUMBITS(6) [],
        VTVM       OFFSET(20)  NUMBITS(1) [],   // VS-mode exception on sfence.vma or satp access
        VTW        OFFSET(21)  NUMBITS(1) [],   // VS-mode exception on wfi
        VTSR       OFFSET(22)  NUMBITS(1) [],   // VS-mode exception on sret
        VSXL       OFFSET(32)  NUMBITS(2) []    // Specify the bit size of VS-mode
    }
);
