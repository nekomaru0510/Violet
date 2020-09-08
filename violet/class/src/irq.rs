pub trait IrqAttr {
    fn new() -> Self;
    fn enable_interrupt(&self, id: u64);
    //fn disable_interrupt(&self, id: u64);
}

