//! Interrupt controller trait

pub trait TraitIntc {
    fn enable_interrupt(&self, id: u32);
    fn disable_interrupt(&self, id: u32);
    fn get_pend_int(&self) -> u32;
    fn set_comp_int(&self, id: u32);
    fn set_prio(&self, id: u32, val: u32);
    fn set_priority_threshold(&self, val: u32);
}
