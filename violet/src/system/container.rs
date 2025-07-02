//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Container management module for the Violet hypervisor.

extern crate alloc;
use alloc::vec::Vec;

use crate::arch::traits::TraitCpu;
use crate::environment::Arch;
use crate::environment::resource::{Resource, ResourceManager}; // [todo delete]
use crate::kernel::Kernel;

/// State of a container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Stopped,
    Running,
}

/// Represents a single container instance.
pub struct Container {
    id: usize, // Container ID (1, 2, 3, ...)
    state: ContainerState, // Current state of the container
    /// Kernel instance for this container
    pub kernel: Kernel,
    /// Resource manager for this container
    pub rm: ResourceManager,
}

impl Container {
    /// Create a new container with the given ID.
    pub fn new(id: usize) -> Self {
        Container {
            id,
            state: ContainerState::Stopped,
            kernel: Kernel::create_custom_kernel(id),
            rm: ResourceManager::new(),
        }
    }

    /// Run the container (set state and run kernel).
    pub fn run(&mut self) {
        self.state = ContainerState::Running;
        self.kernel.run();
    }

    /// Entry point for the container (calls kernel entry).
    pub fn entry(&self) {
        self.kernel.entry();
    }

    /// Returns true if the container is running.
    pub fn is_running(&self) -> bool {
        self.state == ContainerState::Running
    }
}

/// Table for managing all containers.
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

    /// Create a new container and return its ID.
    pub fn create(&mut self) -> usize {
        let id: usize = Self::idx2id(self.containers.len());
        self.containers.push(Container::new(id));
        id
    }

    /// Get a reference to a container by ID (panics if not found).
    pub fn get(&self, id: usize) -> &Container {
        // Do not check id. It is okay to panic for access to other containers.
        &self.containers[Self::id2idx(id)]
    }

    /// Get a mutable reference to a container by ID (panics if not found).
    pub fn get_mut(&mut self, id: usize) -> &mut Container {
        // Do not check id. It is okay to panic for access to other containers.
        &mut self.containers[Self::id2idx(id)]
    }

    /// Returns true if a container with the given ID exists.
    pub fn is_exist(&self, id: usize) -> bool {
        self.containers.get(Self::id2idx(id)).is_some()
    }

    /// Convert container ID to index (ID is 1-based, index is 0-based).
    fn id2idx(id: usize) -> usize {
        id - 1
    }

    /// Convert index to container ID (ID is 1-based, index is 0-based).
    fn idx2id(idx: usize) -> usize {
        idx + 1
    }
}

/// Create a new container and return its ID.
pub fn create_container() -> usize {
    unsafe { CONTAINER_TABLE.create() }
}

/// Returns true if the current container exists.
pub fn does_container_exist() -> bool {
    current_container_id() != 0
}

/// Get a reference to the current container.
pub fn get_container() -> &'static Container {
    unsafe { CONTAINER_TABLE.get(current_container_id()) }
}

/// Get a mutable reference to the current container.
pub fn get_mut_container() -> &'static mut Container {
    unsafe { CONTAINER_TABLE.get_mut(current_container_id()) }
}

/// Get a reference to a container by ID.
pub fn get_container_by_id(id: usize) -> &'static Container {
    unsafe { CONTAINER_TABLE.get(id) }
}

/// Get a mutable reference to a container by ID.
pub fn get_mut_container_by_id(id: usize) -> &'static mut Container {
    unsafe { CONTAINER_TABLE.get_mut(id) }
}

/// Get the current container ID for the running CPU.
pub fn current_container_id() -> usize {
    Arch::get_core().get_container_id()
}

/// Returns true if the container with the given ID exists and is running.
pub fn is_ready_container(id: usize) -> bool {
    unsafe {
        CONTAINER_TABLE.is_exist(id) && get_container_by_id(id).is_running()
    }
}

/// Maximum number of resources per container.
pub const MAX_NUM_OF_RESOURCE: usize = 8;

/// Parameters for creating a container (resource and kernel).
pub struct ContainerParam {
    pub resource: ResourceParam,
    pub kernel: KernelParam,
}

/// Resource parameters for a container.
pub struct ResourceParam {
    pub resource: [Resource; MAX_NUM_OF_RESOURCE],
}

/// Kernel parameters for a container.
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
