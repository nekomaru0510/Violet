//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Timer trait

pub trait TraitTimer {
    fn write(&self, t: u64);
    fn read(&self) -> u64;
    fn enable_interrupt(&self);
    fn disable_interrupt(&self);
    fn set_interrupt_time(&self, t: u64);
}
