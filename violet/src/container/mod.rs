//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Container

extern crate alloc;
use alloc::vec::Vec;

use crate::arch::traits::TraitCpu;
use crate::environment::Arch;
use crate::kernel::Kernel;
use crate::resource::{Resource, ResourceManager}; // [todo delete]

pub enum ContainerState {
    Stopped,
    Running,
}

pub struct Container {
    id: usize,                  // Container ID (1, 2, 3, ...)
    state: ContainerState,
    pub kernel: Kernel,
    pub rm: ResourceManager,
}

impl Container {
    pub fn new(id: usize) -> Self {
        Container {
            id,
            state : ContainerState::Stopped,
            kernel: Kernel::create_custom_kernel(id),
            rm: ResourceManager::new(),
        }
    }

    pub fn run(&mut self) {
        self.state = ContainerState::Running;
        self.kernel.run();
    }

    pub fn entry(&self) {
        self.kernel.entry();
    }

    pub fn is_running(&self) -> bool {
        match self.state {
            ContainerState::Running => true,
            _ => false,
        }
    }
}

/* Container Table */
static mut CONTAINER_TABLE: ContainerTable = ContainerTable::new();

struct ContainerTable {
    containers: Vec<Container>,
}

impl ContainerTable {
    pub const fn new() -> Self {
        ContainerTable {
            containers: Vec::new(),
        }
    }

    pub fn create(&mut self) -> usize {
        let id: usize = Self::idx2id(self.containers.len());
        self.containers.push(Container::new(id));
        id
    }

    pub fn get(&self, id: usize) -> &Container {
        // Do not check id. It is okay to panic for access to other containers.
        &self.containers[Self::id2idx(id)]
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Container {
        // Do not check id. It is okay to panic for access to other containers.
        &mut self.containers[Self::id2idx(id)]
    }

    pub fn is_exist(&self, id: usize) -> bool {
        match self.containers.get(Self::id2idx(id)) {
            Some(_) => true,
            None => false,
        }
    }

    fn id2idx(id: usize) -> usize {
        id - 1
    }

    fn idx2id(idx: usize) -> usize {
        idx + 1
    }
}

pub fn create_container() -> usize {
    unsafe { CONTAINER_TABLE.create() }
}

pub fn does_container_exist() -> bool {
    return !(current_container_id() == 0);
}

pub fn get_container() -> &'static Container {
    unsafe { CONTAINER_TABLE.get(current_container_id()) }
}

pub fn get_mut_container() -> &'static mut Container {
    unsafe { CONTAINER_TABLE.get_mut(current_container_id()) }
}

pub fn get_container_by_id(id: usize) -> &'static Container {
    unsafe { CONTAINER_TABLE.get(id) }
}

pub fn get_mut_container_by_id(id: usize) -> &'static mut Container {
    unsafe { CONTAINER_TABLE.get_mut(id) }
}

pub fn current_container_id() -> usize {
    Arch::get_core().get_container_id()
}

pub fn is_ready_container(id: usize) -> bool {
    unsafe { 
        if CONTAINER_TABLE.is_exist(id) {
            get_container_by_id(id).is_running()
        } else {
            false
        }
    }
}

const MAX_NUM_OF_RESOURCE: usize = 8;

pub struct ContainerParam {
    pub resource: ResourceParam,
    pub kernel: KernelParam,
}

pub struct ResourceParam {
    pub resource: [Resource; MAX_NUM_OF_RESOURCE],
}

pub struct KernelParam {
    root_task: fn(),
    prcid: usize,
}

#[test_case]
fn test_container() -> Result<(), &'static str> {
    create_container();
    get_container();
    Ok(())
}
