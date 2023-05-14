//! イベント(割込みや例外)管理機能

pub struct EventManager {
    
}

pub struct Event {
    
}

pub enum GeneralEventNumber {
    /* 汎用イベント */
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
