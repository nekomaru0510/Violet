//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Trait for general-purpose registers

pub trait TraitRegisters: Copy {
    fn switch(&mut self, regs: &mut Self);
    /*
    fn set(&mut self, idx: usize, );
    fn get(&self, idx: usize) -> ;
    */
}
