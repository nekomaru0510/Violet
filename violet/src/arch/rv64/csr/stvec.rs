//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Supervisor Trap-Vector Base-Address (stvec)

use crate::register;

register!(
    Stvec,              /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x105",   /* Read Instruction */
    "csrw 0x105, {}",   /* Write Instruction */
    {                   /* Register Field */
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    }
);
