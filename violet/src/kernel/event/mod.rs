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
