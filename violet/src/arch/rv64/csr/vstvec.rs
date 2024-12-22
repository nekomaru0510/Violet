//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Virtual Supervisor Trap-Vector Base-Address (vstvec)

use crate::register;

register!(
    Vstvec,             /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x205",   /* Read Instruction */
    "csrw 0x205, {}",   /* Write Instruction */
    {                   /* Register Field */
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    }
);
