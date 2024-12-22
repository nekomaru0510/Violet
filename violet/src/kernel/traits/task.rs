//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Task Trait

pub trait TraitTask {
    fn new(id: u64, func: fn()) -> Self;
    fn get_entry(&self) -> fn();
    fn set_priority(&mut self, prio: u64);
    fn get_priority(&self) -> u64;
}
