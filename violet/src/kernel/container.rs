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

pub struct Container<C, I, T, S>
where
    C: TraitCpu,
    I: TraitIntc,
    T: TraitTimer,
    S: TraitSerial,
{
    pub id: usize,
    pub kernel: Kernel,
    pub cpu: Vec<Option<Box<C>>>,
    pub intc: Option<Box<I>>,
    pub timer: Option<Box<T>>,
    pub serial: Option<Box<S>>,
}

impl<C, I, T, S> Container<C, I, T, S>
where
    C: TraitCpu,
    I: TraitIntc,
    T: TraitTimer,
    S: TraitSerial,
{
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

    pub fn register_cpu(&mut self, cpu: C) {
        self.cpu.push(Some(Box::new(cpu)));
    }

    pub fn register_intc(&mut self, intc: I) {
        self.intc = Some(Box::new(intc));
    }

    pub fn register_timer(&mut self, timer: T) {
        self.timer = Some(Box::new(timer));
    }

    pub fn register_serial(&mut self, serial: S) {
        self.serial = Some(Box::new(serial));
    }
}

pub struct ContainerTable<C, I, T, S>
where
    C: TraitCpu,
    I: TraitIntc,
    T: TraitTimer,
    S: TraitSerial,
{
    containers: Vec<Option<Box<Container<C, I, T, S>>>>,
    cpu2container: [usize; NUM_OF_CPUS],
}

impl<C, I, T, S> ContainerTable<C, I, T, S>
where
    C: TraitCpu,
    I: TraitIntc,
    T: TraitTimer,
    S: TraitSerial,
{
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

    pub fn get(&self, id: usize) -> Option<&Box<Container<C, I, T, S>>> {
        if id < self.containers.len() {
            self.containers[id].as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Box<Container<C, I, T, S>>> {
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
