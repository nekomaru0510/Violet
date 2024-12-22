//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Virtual Supervisor Trap Value Register(vstval)

use crate::register;

register!(
    Vstval,             /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x243",   /* Read Instruction */
    "csrw 0x243, {}",   /* Write Instruction */
    {                   /* Register Field */
        VSTVAL       OFFSET(0)  NUMBITS(64) []
    }
);
