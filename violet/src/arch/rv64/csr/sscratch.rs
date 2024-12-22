//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Supervisor Scratch Register(sscratch)

use crate::register;

register!(
    Sscratch,           /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x140",   /* Read Instruction */
    "csrw 0x140, {}",   /* Write Instruction */
    {                   /* Register Field */
        SSCRATCH       OFFSET(0)  NUMBITS(64) []
    }
);
