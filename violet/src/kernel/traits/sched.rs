//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Scheduler trait

use crate::kernel::traits::task::TraitTask;

pub trait TraitSched<T: TraitTask> {
    fn next(&mut self) -> Option<T>;
    fn register(&mut self, task: T);
    fn unregister(&mut self);
}
