//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Hypervisor Exception Delegation Register (hedeleg)

use crate::register;

register!(
    Hedeleg,            /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x602",   /* Read Instruction */
    "csrw 0x602, {}",   /* Write Instruction */
    {                   /* Register Field */
        HEDELEG       OFFSET(0)  NUMBITS(64) []
    }
);
