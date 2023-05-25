//! 仮想CPU

extern crate alloc;
use alloc::vec::Vec;

use crate::driver::traits::cpu::context::TraitContext;
use crate::environment::NUM_OF_CPUS;
//use crate::driver::traits::cpu::registers::TraitRegisters;
use crate::driver::arch::rv64::get_cpuid; // [todo delete] //test

pub struct VirtualCpuMap<T: TraitContext> {
    vcpus: Vec<VirtualCpu<T>>,
    p2v_cpu: [usize; NUM_OF_CPUS],
}

impl<T: TraitContext> VirtualCpuMap<T> {
    pub const fn new() -> Self {
        VirtualCpuMap {
            vcpus: Vec::new(),
            p2v_cpu: [0; NUM_OF_CPUS],
        }
    }

    pub fn create_vcpu(&mut self, vcpuid: usize, pcpuid: usize) {
        self.p2v_cpu[pcpuid] = vcpuid;
        self.vcpus.push(VirtualCpu::new(vcpuid));
    }

    /* 現在動作している(本メソッドを呼び出したCPUに対応する)仮想CPUのIDを返す */
    pub fn get_vcpuid(&self) -> usize {
        self.p2v_cpu[get_cpuid()]
    }

    pub fn find(&self, vcpuid: usize) -> Option<&VirtualCpu<T>> {
        self.vcpus.iter().find(|e| e.vcpuid == vcpuid)
    }

    pub fn find_mut(&mut self, vcpuid: usize) -> Option<&mut VirtualCpu<T>> {
        self.vcpus.iter_mut().find(|e| e.vcpuid == vcpuid)
    }
}

pub enum VcpuStatus {
    STOPPED,
    RUNNING,
    SUSPENDED,
}

pub struct VirtualCpu<T: TraitContext> {
    vcpuid: usize, /* 仮想CPU番号 */
    pub context: T,
    status: VcpuStatus,
}

impl<T: TraitContext> VirtualCpu<T> {
    pub fn new(vcpuid: usize) -> Self {
        VirtualCpu {
            vcpuid,
            context: T::new(),
            status: VcpuStatus::STOPPED,
        }
    }

    pub fn run(&mut self, regs: &mut T::Registers) {
        // レジスタの復帰
        //self.regs.restore_to(sp);
        self.context.switch(regs);
    }
    /*
    pub fn switch(&mut self) {

    }*/
}

#[cfg(test)]
use crate::driver::arch::rv64::vscontext::VsContext;

#[test_case]
fn test_vcpumap() -> Result<(), &'static str> {
    let mut map = VirtualCpuMap::<VsContext>::new();

    map.create_vcpu(1, 0);

    let result = match map.find(1) {
        None => Err("Fail to find vcpu"),
        Some(x) => Ok(()),
    };

    if result != Ok(()) {
        return result;
    }

    match map.find(0) {
        None => Ok(()),
        Some(x) => Err("Find Invalid vcpu"),
    }
}
