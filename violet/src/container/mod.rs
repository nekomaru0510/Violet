//! Container

extern crate alloc;
use alloc::vec::Vec;

use crate::arch::traits::TraitArch;
use crate::environment::Arch;
use crate::kernel::Kernel;
use crate::resource::{Resource, ResourceManager}; // [todo delete]

use crate::environment::NUM_OF_CPUS;

pub struct Container {
    id: usize,
    pub kernel: Kernel,
    pub rm: ResourceManager,
}

impl Container {
    pub fn new(id: usize) -> Self {
        Container {
            id,
            kernel: Kernel::create_custom_kernel(id),
            rm: ResourceManager::new(),
        }
    }
}

/* Container Table */
static mut CONTAINER_TABLE: ContainerTable = ContainerTable::new();

struct ContainerTable {
    containers: Vec<Container>,
    cpu2container: [usize; NUM_OF_CPUS],
}

impl ContainerTable {
    pub const fn new() -> Self {
        ContainerTable {
            containers: Vec::new(),
            cpu2container: [0; NUM_OF_CPUS],
        }
    }

    pub fn create(&mut self) -> usize {
        let id: usize = self.containers.len();
        self.containers.push(Container::new(id));
        id
    }

    pub fn get(&self, id: usize) -> &Container {
        // Do not check id. It is okay to panic for access to other containers.
        &self.containers[id]
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Container {
        // Do not check id. It is okay to panic for access to other containers.
        &mut self.containers[id]
    }

    pub fn current_id(&self) -> usize {
        self.cpu2container[Arch::get_cpuid()]
    }

    pub fn is_ready(&self) -> bool {
        if self.containers.len() == 0 {
            false
        } else {
            true
        }
    }
}

pub fn create_container() -> usize {
    unsafe { CONTAINER_TABLE.create() }
}

pub fn get_container() -> &'static Container {
    unsafe { CONTAINER_TABLE.get(current_container_id()) }
}

pub fn get_mut_container() -> &'static mut Container {
    unsafe { CONTAINER_TABLE.get_mut(current_container_id()) }
}

pub fn current_container_id() -> usize {
    unsafe { CONTAINER_TABLE.current_id() }
}

pub fn is_ready_container() -> bool {
    unsafe { CONTAINER_TABLE.is_ready() }
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
