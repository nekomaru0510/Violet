//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Dispatcher trait
use crate::kernel::traits::task::TraitTask;

pub trait TraitDispatcher<T: TraitTask> {
    fn dispatch(&self, task: &T);
}
