//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Supervisor Exception Program Counter(sepc)

use crate::register;

register!(
    Sepc,               /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x141",   /* Read Instruction */
    "csrw 0x141, {}",   /* Write Instruction */
    {                   /* Register Field */
        SEPC       OFFSET(0)  NUMBITS(64) []
    }
);
