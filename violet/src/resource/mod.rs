//! Resource Manager
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::container::{get_container, get_mut_container};

use crate::arch::traits::TraitCpu;
use crate::driver::traits::intc::TraitIntc;
use crate::driver::traits::serial::TraitSerial;
use crate::driver::traits::timer::TraitTimer;

pub struct ResourceManager {
    cpu: Vec<Box<dyn TraitCpu>>,
    intc: Vec<Box<dyn TraitIntc>>,
    timer: Vec<Box<dyn TraitTimer>>,
    serial: Vec<Box<dyn TraitSerial>>,
    /* memory */
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            cpu: Vec::new(),
            intc: Vec::new(),
            timer: Vec::new(),
            serial: Vec::new(),
        }
    }

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

    pub fn get(&self, devtype: ResourceType, idx: usize) -> BorrowResource {
        match devtype {
            ResourceType::Cpu => {
                if self.cpu.len() < idx {
                    BorrowResource::None
                } else {
                    BorrowResource::Cpu(&self.cpu[idx])
                }
            }
            ResourceType::Intc => {
                if self.intc.len() < idx {
                    BorrowResource::None
                } else {
                    BorrowResource::Intc(&self.intc[idx])
                }
            }
            ResourceType::Timer => {
                if self.timer.len() < idx {
                    BorrowResource::None
                } else {
                    BorrowResource::Timer(&self.timer[idx])
                }
            }
            ResourceType::Serial => {
                if self.serial.len() < idx {
                    BorrowResource::None
                } else {
                    BorrowResource::Serial(&self.serial[idx])
                }
            }
            _ => BorrowResource::None,
        }
    }

    pub fn get_mut(&mut self, devtype: ResourceType, idx: usize) -> BorrowMutResource {
        match devtype {
            ResourceType::Cpu => {
                if self.cpu.len() < idx {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Cpu(&mut self.cpu[idx])
                }
            }
            ResourceType::Intc => {
                if self.intc.len() < idx {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Intc(&mut self.intc[idx])
                }
            }
            ResourceType::Timer => {
                if self.timer.len() < idx {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Timer(&mut self.timer[idx])
                }
            }
            ResourceType::Serial => {
                if self.serial.len() < idx {
                    BorrowMutResource::None
                } else {
                    BorrowMutResource::Serial(&mut self.serial[idx])
                }
            }
            _ => BorrowMutResource::None,
        }
    }
}

pub enum ResourceType {
    Cpu,
    Intc,
    Timer,
    Serial,
    None,
}

pub enum Resource {
    Cpu(Box<dyn TraitCpu>),
    Intc(Box<dyn TraitIntc>),
    Timer(Box<dyn TraitTimer>),
    Serial(Box<dyn TraitSerial>),
    None,
}

pub enum BorrowResource<'a> {
    Cpu(&'a Box<dyn TraitCpu>),
    Intc(&'a Box<dyn TraitIntc>),
    Timer(&'a Box<dyn TraitTimer>),
    Serial(&'a Box<dyn TraitSerial>),
    None,
}

pub enum BorrowMutResource<'a> {
    Cpu(&'a mut Box<dyn TraitCpu>),
    Intc(&'a mut Box<dyn TraitIntc>),
    Timer(&'a mut Box<dyn TraitTimer>),
    Serial(&'a mut Box<dyn TraitSerial>),
    None,
}

pub fn get_resources() -> &'static ResourceManager {
    &get_container().rm
}

pub fn get_mut_resources() -> &'static mut ResourceManager {
    &mut get_mut_container().rm
}

#[cfg(test)]
use crate::driver::board::sifive_u::uart::Uart;

#[test_case]
fn test_rm() -> Result<(), &'static str> {
    let mut rm = ResourceManager::new();
    let idx = rm.register(Resource::Serial(Box::new(Uart::new(0x1000_0000))));

    if let BorrowResource::Serial(x) = rm.get(ResourceType::Serial, 0) {
        Ok(())
    } else {
        Err("Failed to open Resource")
    }
}

/*
#[cfg(test)]
use crate::driver::board::sifive_u::uart::Uart;
#[cfg(test)]
use resource::{ResourceType, Resource, BorrowResource};
#[test_case]
fn test_container() -> Result<(), &'static str> {
    create_container();
    get_container();
    get_mut_resources().register(Resource::Serial(Box::new(Uart::new(0x1000_0000))));

    if let BorrowResource::Serial(x) = get_resources().get(ResourceType::Serial, 0) {
        x.write('s' as u8);
        Ok(())
    } else {
        Err("Failed to open Resource")
    }

}
*/
