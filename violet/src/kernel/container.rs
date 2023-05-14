//! Violetコンテナ
extern crate alloc;
use alloc::boxed::Box;
//use alloc::rc::Rc;
use alloc::vec::Vec;

use crate::environment::NUM_OF_CPUS;

use crate::driver::traits::cpu::TraitCpu;
use crate::driver::traits::intc::TraitIntc;
use crate::driver::traits::serial::TraitSerial;
use crate::driver::traits::timer::TraitTimer;
use crate::kernel::Kernel;

pub struct Container {
    pub id: usize,
    pub kernel: Kernel,
    pub cpu: Vec<Option<Box<dyn TraitCpu>>>,
    pub serial: Option<Box<dyn TraitSerial>>,
    pub intc: Option<Box<dyn TraitIntc>>,
    pub timer: Option<Box<dyn TraitTimer>>,
}

impl Container {
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

    pub fn register_cpu<T: TraitCpu + 'static>(&mut self, cpu: T) {
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
}

static mut CONTAINER_TABLE: Vec<Option<Box<Container>>> = Vec::new();

pub fn create_container() -> usize {
    unsafe {
        let id: usize = CONTAINER_TABLE.len();
        CONTAINER_TABLE.push(Some(Box::new(Container::new(id))));
        id
    }
}

pub fn get_mut_container(id: usize) -> Option<&'static mut Box<Container>> {
    unsafe {
        if (id) < CONTAINER_TABLE.len() {
            CONTAINER_TABLE[0].as_mut()
        } else {
            None
        }
    }
}

pub fn get_container(id: usize) -> Option<&'static Box<Container>> {
    unsafe {
        if (id) < CONTAINER_TABLE.len() {
            CONTAINER_TABLE[0].as_ref()
        } else {
            None
        }
    }
}

/* CPU番号からコンテナ番号を取得する */
static mut CPU_CONTAINER_MAP: [usize; NUM_OF_CPUS] = [0; NUM_OF_CPUS];

use crate::driver::arch::rv64::get_cpuid; // [todo delete] //test
pub fn current_container_id() -> usize {
    unsafe { CPU_CONTAINER_MAP[get_cpuid()] }
}

pub fn current_container() -> Option<&'static Box<Container>> {
    get_container(current_container_id())
}

pub fn current_mut_container() -> Option<&'static mut Box<Container>> {
    get_mut_container(current_container_id())
}

extern crate core;
use core::intrinsics::transmute;

/* コンテナ隠ぺい用関数 */
pub fn cpu_mut() -> &'static mut dyn TraitCpu {
    let con = get_mut_container(current_container_id());
    match con {
        /* [todo fix]コンテナが複数あった際に、cpuのインデックス=cpuidとならない */
        Some(c) => { 
            match c.cpu[get_cpuid()].as_mut() {
                None => {
                    panic!("Nothing Cpu in Container");
                },
                Some(p) => {
                    unsafe {transmute(p.as_mut()) }
                }
            }
        }, 
        None => {
            panic!("Nothing Container");
        },
    }
}

#[cfg(test)]
use crate::driver::board::sifive_u::uart::Uart;
#[cfg(test)]
use crate::driver::arch::rv64::Rv64;

#[test_case]
fn test_context() -> Result<(), &'static str> {
    create_container();
    let con = get_mut_container(0);
    match con {
        Some(c) => c.register_cpu(Rv64::new(0)),
        None => (),
    }
    //cpu_mut::<Rv64>().wakeup();
    cpu_mut().wakeup();

    unsafe{ transmute::<&'static mut dyn TraitCpu, &mut Rv64>(cpu_mut()).set_default_vector();}

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
