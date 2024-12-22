//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Evnet(interrupt or exception) manager

pub struct EventManager {
    
}

pub struct Event {
    
}

pub enum GeneralEventNumber {
    // General event
    NoneEvent = 0,
    Nmi,
    Ipi,
    TimerInterrupt,
    ExternalInterrupt,
    PageFault,
    UnknownInstruction,
    /*  */
    GeneralEvent = 32,
}
