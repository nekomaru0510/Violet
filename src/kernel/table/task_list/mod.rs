//! task_list module

use crate::app::App;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/* 
pub trait TaskList {
    pub fn new() -> Self;
    pub fn register(&self, name: String);
}
*/

pub struct TaskList {
    current: u64,
    tasks: Vec<Task>,
}

struct Task<'a, T: App> {
    id: u64,
    app: &'a T,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList{current: 0, tasks: Vec::new(), }
    }

    pub fn register(&mut self, app: App) {
        let task = Task::new(self.tasks.len() as u64, app) ; //もう少し真面目にTask IDの割当て方を考える
        self.tasks.push(task);
    }

}

impl Task {
    pub fn new(id: u64, app: App) -> Self {
        Task {id, app}
    }
}