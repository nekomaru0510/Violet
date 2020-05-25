//! Resource Module

pub mod cpu;
use cpu::Cpu;

pub mod io;
use io::IO;

pub mod irq;
use irq::Irq;

pub mod handler_table;
use handler_table::HandlerTable;

pub struct Resource {
    pub io: IO,
    pub cpu: Cpu,
    pub irq: Irq,
    pub htable: HandlerTable,
}

impl Resource {
    pub fn new() -> Self {
        Resource {io: IO::new(), cpu: Cpu::new(), irq: Irq::new(), htable: HandlerTable::new(), }
    }
}


