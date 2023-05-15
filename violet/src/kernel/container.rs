//! Violetコンテナ
extern crate alloc;
use alloc::boxed::Box;
//use alloc::rc::Rc;
use alloc::vec::Vec;

use crate::environment::NUM_OF_CPUS;

use crate::driver::arch::rv64::get_cpuid; // [todo delete] //test
use crate::driver::traits::cpu::TraitCpu;
use crate::driver::traits::intc::TraitIntc;
use crate::driver::traits::serial::TraitSerial;
use crate::driver::traits::timer::TraitTimer;
use crate::kernel::Kernel;

pub struct Container<C: TraitCpu> {
    pub id: usize,
    pub kernel: Kernel,
    pub cpu: Vec<Option<Box<C>>>,
    pub serial: Option<Box<dyn TraitSerial>>,
    pub intc: Option<Box<dyn TraitIntc>>,
    pub timer: Option<Box<dyn TraitTimer>>,
}

impl<C: TraitCpu> Container<C> {
    pub fn new(id: usize) -> Self {
        Container {
            id,
            kernel: Kernel::create_custom_kernel(id),
            cpu: Vec::new(),
            intc: None,
            serial: None,
            timer: None,
        }
    }

    //pub fn register_cpu<T: TraitCpu + 'static>(&mut self, cpu: T) {
    pub fn register_cpu(&mut self, cpu: C) {
        self.cpu.push(Some(Box::new(cpu)));
    }

    pub fn register_intc<T: TraitIntc + 'static>(&mut self, intc: T) {
        self.intc = Some(Box::new(intc));
    }

    pub fn register_serial<T: TraitSerial + 'static>(&mut self, serial: T) {
        self.serial = Some(Box::new(serial));
    }

    pub fn register_timer<T: TraitTimer + 'static>(&mut self, timer: T) {
        self.timer = Some(Box::new(timer));
    }

    /*
    pub fn get_intc<>

    pub fn get_device<T>(&self, p: Peripheral) -> Option<Box<dyn T>> {
        match p {
            InterruptController => self.intc,
            Serial => self.serial,
            Timer => self.timer,
            _ => None
        }
    }
    */
}

pub enum Peripheral {
    InterruptController = 1,
    Serial,
    Timer,
    CustomPeripheral = 128,
}

pub struct ContainerTable<T: TraitCpu> {
    containers: Vec<Option<Box<Container<T>>>>,
    cpu2container: [usize; NUM_OF_CPUS],
}

impl<T: TraitCpu> ContainerTable<T> {
    pub const fn new() -> Self {
        ContainerTable {
            containers: Vec::new(),
            cpu2container: [0; NUM_OF_CPUS],
        }
    }

    pub fn add(&mut self) -> usize {
        let id: usize = self.containers.len();
        self.containers.push(Some(Box::new(Container::new(id))));
        id
    }

    pub fn get(&self, id: usize) -> Option<&Box<Container<T>>> {
        if id < self.containers.len() {
            self.containers[id].as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Box<Container<T>>> {
        if id < self.containers.len() {
            self.containers[id].as_mut()
        } else {
            None
        }
    }

    pub fn current_id(&self) -> usize {
        self.cpu2container[get_cpuid()]
    }
}

/*
#[cfg(test)]
use crate::driver::board::sifive_u::uart::Uart;


#[test_case]
fn test_context() -> Result<(), &'static str> {
    create_container();

    let uart = Uart::new(0x1000_0000);
    let con = get_mut_container(0);
    match con {
        Some(c) => c.register_serial(uart),
        None => (),
    }

    let con = get_container(0);
    match &con.unwrap().serial {
        None => (),
        Some(s) => s.write('s' as u8),
    }

    match &con.unwrap().cpu[0] {
        None => (),
        Some(c) => c.enable_interrupt(),
    }

    Ok(())
}
*/
