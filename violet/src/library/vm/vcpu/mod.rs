//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Virtual CPU

pub mod vreg;

extern crate alloc;
use alloc::vec::Vec;

use crate::arch::traits::context::TraitContext;
use crate::arch::traits::hypervisor::HypervisorT;
use crate::arch::traits::TraitArch;
use vreg::{VirtualRegisterMap, VirtualRegisterT};

use crate::environment::Hyp;
use crate::environment::Arch;

pub struct VirtualCpuMap {
    vcpus: Vec<VirtualCpu>,
    /// Mapping from physical CPU ID to vCPU ID (per VM, dynamic size)
    p2v_cpu: Vec<Option<usize>>,
}

impl VirtualCpuMap {
    pub fn new() -> Self {
        VirtualCpuMap {
            vcpus: Vec::new(),
            p2v_cpu: Vec::new(),
        }
    }

    pub fn register(&mut self, vcpuid: usize, pcpuid: usize) {
        // Dynamically resize mapping if needed
        if self.p2v_cpu.len() <= pcpuid {
            self.p2v_cpu.resize(pcpuid + 1, None);
        }
        self.p2v_cpu[pcpuid] = Some(vcpuid);
        self.vcpus.push(VirtualCpu::new(vcpuid));
    }

    /// Return the ID of the virtual CPU currently running
    pub fn get_vcpuid(&self) -> usize {
        let pcpuid = Arch::get_cpuid();
        self.p2v_cpu.get(pcpuid).and_then(|x| *x).unwrap_or(0)
    }

    pub fn get(&self, vcpuid: usize) -> Option<&VirtualCpu> {
        self.find(vcpuid)
    }

    pub fn get_mut(&mut self, vcpuid: usize) -> Option<&mut VirtualCpu> {
        self.find_mut(vcpuid)
    }

    pub fn find(&self, vcpuid: usize) -> Option<&VirtualCpu> {
        self.vcpus.iter().find(|e| e.vcpuid == vcpuid)
    }

    pub fn find_mut(&mut self, vcpuid: usize) -> Option<&mut VirtualCpu> {
        self.vcpus.iter_mut().find(|e| e.vcpuid == vcpuid)
    }

    pub fn len(&self) -> usize {
        self.vcpus.len()
    }
}

pub enum VcpuStatus {
    STOPPED,
    RUNNING,
    SUSPENDED,
}

pub struct VirtualCpu {
    vcpuid: usize,
    pub context: <Hyp as HypervisorT>::Context,
    status: VcpuStatus,
    pub vregs: VirtualRegisterMap,
}

impl VirtualCpu {
    pub fn new(vcpuid: usize) -> Self {
        VirtualCpu {
            vcpuid,
            context: <Hyp as HypervisorT>::Context::new(),
            status: VcpuStatus::STOPPED,
            vregs: VirtualRegisterMap::new(),
        }
    }

    pub fn run(&mut self, regs: &mut <<Hyp as HypervisorT>::Context as TraitContext>::Registers) {
        // Restore registers
        self.context.switch(regs);
    }

    pub fn set_vcpuid(&mut self, vcpuid: usize) {
        self.vcpuid = vcpuid;
    }

    pub fn get_vcpuid(&self) -> usize {
        self.vcpuid
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
        Hyp::hook(vecid, func);
    }
}

#[test_case]
fn test_vcpumap() -> Result<(), &'static str> {
    let mut map = VirtualCpuMap::new();

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
use vreg::vmhartid::Vreg;

#[test_case]
fn test_vcpu() -> Result<(), &'static str> {
    let mut vcpu = VirtualCpu::new(0);
    let vreg = Vreg::new(0x128);

    vcpu.register(1, vreg);
    if vcpu.read(1) == Some(0x128) {
        return Ok(());
    } else {
        return Err("Failed to read virtual register");
    }
}