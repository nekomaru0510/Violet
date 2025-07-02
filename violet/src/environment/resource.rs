// SPDX-License-Identifier: MIT 
// SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Resource Manager
//! 
//! This module provides resource management for CPUs, interrupt controllers, timers, and serial devices.
//! It is used by each container to register and access hardware resources in a type-safe manner.

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::system::container::{get_container, get_mut_container};
use crate::arch::traits::TraitCpu;
use crate::driver::traits::intc::TraitIntc;
use crate::driver::traits::serial::TraitSerial;
use crate::driver::traits::timer::TraitTimer;

/// Manages hardware resources for a container.
///
/// Each resource type is managed as a vector of trait objects.
pub struct ResourceManager {
    /// List of CPU resources
    cpu: Vec<&'static dyn TraitCpu>,
    /// List of interrupt controller resources
    intc: Vec<Box<dyn TraitIntc>>,
    /// List of timer resources
    timer: Vec<Box<dyn TraitTimer>>,
    /// List of serial port resources
    serial: Vec<Box<dyn TraitSerial>>,
    // TODO: Add memory and other resources as needed
}

impl ResourceManager {
    /// Create a new, empty resource manager.
    pub fn new() -> Self {
        ResourceManager {
            cpu: Vec::new(),
            intc: Vec::new(),
            timer: Vec::new(),
            serial: Vec::new(),
        }
    }

    /// Register a resource and return its index.
    ///
    /// Returns the index of the resource in its vector, or an error if the type is unsupported.
    pub fn register(&mut self, dev: Resource) -> Result<usize, ()> {
        match dev {
            Resource::Cpu(c) => {
                self.cpu.push(c);
                Ok(self.cpu.len() - 1)
            }
            Resource::Intc(i) => {
                self.intc.push(i);
                Ok(self.intc.len() - 1)
            }
            Resource::Timer(t) => {
                self.timer.push(t);
                Ok(self.timer.len() - 1)
            }
            Resource::Serial(s) => {
                self.serial.push(s);
                Ok(self.serial.len() - 1)
            }
            _ => Err(()),
        }
    }

    /// Get an immutable reference to a resource by type and index.
    pub fn get(&self, devtype: ResourceType, idx: usize) -> BorrowResource {
        match devtype {
            ResourceType::Cpu => {
                if idx >= self.cpu.len() {
                    BorrowResource::None
                } else {
                    BorrowResource::Cpu(&self.cpu[idx])
                }
            }
            ResourceType::Intc => {
                if idx >= self.intc.len() {
                    BorrowResource::None
                } else {
                    BorrowResource::Intc(&self.intc[idx])
                }
            }
            ResourceType::Timer => {
                if idx >= self.timer.len() {
                    BorrowResource::None
                } else {
                    BorrowResource::Timer(&self.timer[idx])
                }
            }
            ResourceType::Serial => {
                if idx >= self.serial.len() {
                    BorrowResource::None
                } else {
                    BorrowResource::Serial(&self.serial[idx])
                }
            }
            _ => BorrowResource::None,
        }
    }

    /// Get a mutable reference to a resource by type and index.
    pub fn get_mut(&mut self, devtype: ResourceType, idx: usize) -> BorrowMutResource {
        match devtype {
            ResourceType::Cpu => {
                if idx >= self.cpu.len() {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Cpu(&mut self.cpu[idx])
                }
            }
            ResourceType::Intc => {
                if idx >= self.intc.len() {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Intc(&mut self.intc[idx])
                }
            }
            ResourceType::Timer => {
                if idx >= self.timer.len() {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Timer(&mut self.timer[idx])
                }
            }
            ResourceType::Serial => {
                if idx >= self.serial.len() {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Serial(&mut self.serial[idx])
                }
            }
            _ => BorrowMutResource::None,
        }
    }
}

/// Resource type identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    /// CPU resource
    Cpu,
    /// Interrupt controller resource
    Intc,
    /// Timer resource
    Timer,
    /// Serial port resource
    Serial,
    /// No resource (invalid or uninitialized)
    None,
}

/// Resource trait object wrapper.
pub enum Resource {
    /// CPU resource
    Cpu(&'static dyn TraitCpu),
    /// Interrupt controller resource
    Intc(Box<dyn TraitIntc>),
    /// Timer resource
    Timer(Box<dyn TraitTimer>),
    /// Serial port resource
    Serial(Box<dyn TraitSerial>),
    /// No resource (invalid or uninitialized)
    None,
}

/// Immutable borrow of a resource.
pub enum BorrowResource<'a> {
    /// CPU resource
    Cpu(&'a &'static dyn TraitCpu),
    /// Interrupt controller resource
    Intc(&'a Box<dyn TraitIntc>),
    /// Timer resource
    Timer(&'a Box<dyn TraitTimer>),
    /// Serial port resource
    Serial(&'a Box<dyn TraitSerial>),
    /// No resource (invalid or uninitialized)
    None,
}

/// Mutable borrow of a resource.
pub enum BorrowMutResource<'a> {
    /// CPU resource
    Cpu(&'a mut &'static dyn TraitCpu),
    /// Interrupt controller resource
    Intc(&'a mut Box<dyn TraitIntc>),
    /// Timer resource
    Timer(&'a mut Box<dyn TraitTimer>),
    /// Serial port resource
    Serial(&'a mut Box<dyn TraitSerial>),
    /// No resource (invalid or uninitialized)
    None,
}

/// Get the resource manager for the current container (immutable).
pub fn get_resources() -> &'static ResourceManager {
    &get_container().rm
}

/// Get the resource manager for the current container (mutable).
pub fn get_mut_resources() -> &'static mut ResourceManager {
    &mut get_mut_container().rm
}

#[cfg(test)]
use crate::driver::board::sifive_u::uart::Uart;

/// Test: Register and retrieve a serial resource.
#[test_case]
fn test_rm() -> Result<(), &'static str> {
    let mut rm = ResourceManager::new();
    let idx = rm.register(Resource::Serial(Box::new(Uart::new(0x1000_0000))));
    assert!(idx.is_ok(), "Failed to register serial resource");
    match rm.get(ResourceType::Serial, 0) {
        BorrowResource::Serial(_) => Ok(()),
        _ => Err("Failed to open Resource"),
    }
}

/// Test: Out-of-bounds access returns None.
#[test_case]
fn test_rm_out_of_bounds() -> Result<(), &'static str> {
    let rm = ResourceManager::new();
    match rm.get(ResourceType::Serial, 0) {
        BorrowResource::None => Ok(()),
        _ => Err("Expected None for out-of-bounds access"),
    }
}
