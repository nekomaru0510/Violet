//! 仮想CPU

pub mod vreg;

extern crate alloc;
use alloc::vec::Vec;

use crate::arch::traits::context::TraitContext;
use crate::arch::traits::hypervisor::HypervisorT;
use crate::environment::NUM_OF_CPUS;
//use crate::arch::traits::registers::TraitRegisters;
use crate::arch::rv64::get_cpuid; // [todo delete] //test
use vreg::{VirtualRegisterMap, VirtualRegisterT};

pub struct VirtualCpuMap<T: HypervisorT> {
    vcpus: Vec<VirtualCpu<T>>,
    p2v_cpu: [usize; NUM_OF_CPUS],
}

impl<T: HypervisorT> VirtualCpuMap<T> {
    pub const fn new() -> Self {
        VirtualCpuMap {
            vcpus: Vec::new(),
            p2v_cpu: [0; NUM_OF_CPUS],
        }
    }

    pub fn register(&mut self, vcpuid: usize, pcpuid: usize) {
        self.p2v_cpu[pcpuid] = vcpuid;
        self.vcpus.push(VirtualCpu::new(vcpuid));
    }

    /* 現在動作している(本メソッドを呼び出したCPUに対応する)仮想CPUのIDを返す */
    pub fn get_vcpuid(&self) -> usize {
        self.p2v_cpu[get_cpuid()]
    }

    pub fn get(&self, vcpuid: usize) -> Option<&VirtualCpu<T>> {
        self.find(vcpuid)
    }

    pub fn get_mut(&mut self, vcpuid: usize) -> Option<&mut VirtualCpu<T>> {
        self.find_mut(vcpuid)
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

pub struct VirtualCpu<T: HypervisorT> {
    vcpuid: usize, /* 仮想CPU番号 */
    pub context: T::Context,
    status: VcpuStatus,
    vregs: VirtualRegisterMap,
}

impl<T: HypervisorT> VirtualCpu<T> {
    pub fn new(vcpuid: usize) -> Self {
        VirtualCpu {
            vcpuid,
            context: T::Context::new(),
            status: VcpuStatus::STOPPED,
            vregs: VirtualRegisterMap::new(),
        }
    }

    pub fn run(&mut self, regs: &mut <T::Context as TraitContext>::Registers) {
        // レジスタの復帰
        //self.regs.restore_to(sp);
        self.context.switch(regs);
    }

    pub fn register<U: VirtualRegisterT + 'static>(&mut self, id: usize, vreg: U) {
        self.vregs.register(id, vreg);
    }

    pub fn read(&mut self, id: usize) -> Option<u64> {
        match self.vregs.get_mut(id) {
            None => None,
            Some(r) => Some(r.read()),
        }
    }

    pub fn write(&mut self, id: usize, val: u64) {
        match self.vregs.get_mut(id) {
            None => (),
            Some(r) => r.write(val),
        }
    }

    pub fn hook(&self, vecid: usize, func: fn(regs: *mut usize)) {
        T::hook(vecid, func);
    }

    /*
    pub fn switch(&mut self) {

    }*/
}

#[cfg(test)]
use crate::driver::arch::rv64::extension::hypervisor::Hext;

#[test_case]
fn test_vcpumap() -> Result<(), &'static str> {
    let mut map = VirtualCpuMap::<Hext>::new();

    map.register(1, 0);

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

#[cfg(test)]
use vreg::vmhartid::Vmhartid;

#[test_case]
fn test_vcpu() -> Result<(), &'static str> {
    let mut vcpu = VirtualCpu::<Hext>::new(0);
    let vmhartid = Vmhartid::new(0x128);

    vcpu.register(1, vmhartid);
    if vcpu.read(1) == Some(0x128) {
        return Ok(());
    } else {
        return Err("Failed to read virtual register");
    }
}
