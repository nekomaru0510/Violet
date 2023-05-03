//! VirtualMachine

pub mod mm;
pub mod vcpu;
pub mod vdev;
pub mod vmem;

extern crate alloc;
use alloc::boxed::Box;

use crate::CPU;

use crate::driver::arch::rv64::mmu::sv48::PageTableSv48;
use crate::driver::arch::rv64::{Exception, Interrupt, PagingMode, TraitRisvCpu};
use crate::driver::traits::cpu::TraitCpu;

use mm::*; /* [todo delete] */
extern crate core;
use core::intrinsics::transmute;

pub fn setup_boot() {
    CPU.switch_hs_mode();

    CPU.enable_interrupt();
    CPU.set_default_vector();

    enable_paging();

    CPU.int.disable_mask_s(
        Interrupt::SupervisorSoftwareInterrupt.mask()
            | Interrupt::SupervisorTimerInterrupt.mask()
            | Interrupt::SupervisorExternalInterrupt.mask(),
    );

    CPU.int.enable_mask_s(
        Interrupt::VirtualSupervisorSoftwareInterrupt.mask()
            | Interrupt::VirtualSupervisorTimerInterrupt.mask()
            | Interrupt::VirtualSupervisorExternalInterrupt.mask()
            | Interrupt::SupervisorGuestExternalInterrupt.mask(),
    );

    CPU.hyp.set_delegation_exc(
        Exception::InstructionAddressMisaligned.mask()
            | Exception::Breakpoint.mask()
            | Exception::EnvironmentCallFromUmodeOrVUmode.mask()
            | Exception::InstructionPageFault.mask()
            | Exception::LoadPageFault.mask()
            | Exception::StoreAmoPageFault.mask(),
    );

    CPU.hyp.set_delegation_int(
        Interrupt::VirtualSupervisorSoftwareInterrupt.mask()
            | Interrupt::VirtualSupervisorTimerInterrupt.mask()
            | Interrupt::VirtualSupervisorExternalInterrupt.mask(),
    );

    CPU.hyp.flush_vsmode_interrupt(0xffff_ffff_ffff_ffff);

    CPU.mmu.set_paging_mode(PagingMode::Bare);

    CPU.hyp.enable_vsmode_counter_access(0xffff_ffff);
}

use vcpu::VirtualCpu;
use vcpu::VirtualCpuMap;
use vdev::VirtualDevice;
use vdev::VirtualIoMap;
use vmem::VirtualMemoryMap;

use crate::driver::arch::rv64::vscontext::VsContext;
use crate::driver::traits::cpu::context::TraitContext; //[todo delete]

pub struct VirtualMachine {
    vcpumap: VirtualCpuMap<VsContext>, /* [todo fix]ここでコンテキストの型を指定したくないが、ジェネリクスにもしたくない */
    vmem: VirtualMemoryMap,
    viomap: Option<VirtualIoMap>,
}

impl VirtualMachine {
    pub const fn new() -> VirtualMachine {
        VirtualMachine {
            vcpumap: VirtualCpuMap::<VsContext>::new(),
            vmem: VirtualMemoryMap::new(),
            viomap: None,
        }
    }

    pub fn setup(&self) {
        /* ゲスト起動前のデフォルトセットアップ */
        setup_boot();
    }

    pub fn run(&mut self) {
        match self.vcpu(self.vcpumap.get_vcpuid()) {
            None => (),
            Some(v) => v.context.jump(),
        };
    }

    pub fn register_cpu(&mut self, vcpuid: usize, pcpuid: usize) {
        self.vcpumap.create_vcpu(vcpuid, pcpuid);
    }

    pub fn vcpu(&self, vcpuid: usize) -> Option<&VirtualCpu<VsContext>> {
        self.vcpumap.find(vcpuid)
    }

    pub fn vcpu_mut(&mut self, vcpuid: usize) -> Option<&mut VirtualCpu<VsContext>> {
        self.vcpumap.find_mut(vcpuid)
    }

    /*
    pub fn map_all_guest_page(&mut self) {

    }*/

    pub fn register_mem(&mut self, vaddr: usize, paddr: usize, size: usize) {
        self.vmem.register(vaddr, paddr, size);
    }

    /*
    pub fn unregister_mem(&mut self, ) {

    }
    */

    pub fn map_guest_page(&mut self, guest_paddr: usize) {
        match self.vmem.get(guest_paddr) {
            None => {}
            Some(m) => {
                match m.get_paddr(guest_paddr) {
                    None => {}
                    Some(r) => {
                        /* [todo fix] CPUトレイトから呼び出す */
                        map_vaddr::<PageTableSv48>(
                            unsafe { transmute(CPU.hyp.get_hs_pagetable()) },
                            r,
                            guest_paddr,
                        );
                    }
                }
            }
        }
    }

    pub fn register_dev<T: VirtualDevice + 'static>(&mut self, base: usize, size: usize, vdev: T) {
        match &mut self.viomap {
            None => {
                self.viomap = Some(VirtualIoMap::new());
                self.viomap.as_mut().unwrap().register(base, size, vdev);
            }
            Some(v) => {
                v.register(base, size, vdev);
            }
        }
    }

    pub fn unregister_dev<T: VirtualDevice + 'static>(
        &mut self,
        base_addr: usize,
        size: usize,
        vdev: T,
    ) {
        // [todo fix] 実装する
    }

    pub fn get_dev_mut(&mut self, addr: usize) -> Option<&mut Box<dyn VirtualDevice>> {
        match &mut self.viomap {
            None => None,
            Some(v) => v.get_mut(addr),
        }
    }

    pub fn write_dev(&mut self, addr: usize, val: usize) -> Option<()> {
        match self.get_dev_mut(addr) {
            None => None,
            Some(d) => {
                d.write(addr, val);
                Some(())
            }
        }
    }

    pub fn read_dev(&mut self, addr: usize) -> Option<usize> {
        match self.get_dev_mut(addr) {
            None => None,
            Some(d) => Some(d.read(addr) as usize),
        }
    }
}

#[cfg(test)]
use crate::system::vm::vdev::vplic::VPlic;

#[test_case]
fn test_read_write_dev() -> Result<(), &'static str> {
    let mut vm: VirtualMachine = VirtualMachine::new();
    let vplic = VPlic::new();
    let val = 0x01;
    vm.register_dev(0x0c00_0000, 0x0400_0000, vplic);

    let mut result = match vm.write_dev(0xc00_0000, val) {
        None => Err("can't write virtual device"),
        Some(x) => Ok(()),
    };
    if result != Ok(()) {
        return result;
    };

    result = match vm.read_dev(0xc00_0000) {
        None => Err("can't read virtual device"),
        Some(x) => {
            if x == val {
                Ok(())
            } else {
                Err("Invalid value")
            }
        }
    };

    result
}

#[cfg(test)]
use crate::driver::arch::rv64::vscontext::*; //[todo delete]

#[test_case]
fn test_vcpu() -> Result<(), &'static str> {
    let mut vm: VirtualMachine = VirtualMachine::new();
    // ブート
    // 自動で自分のCPU番号から仮想CPUを取得
    vm.register_cpu(1, 0);
    match vm.vcpu_mut(1) {
        None => (),
        Some(v) => {
            v.context.set(JUMP_ADDR /*EPC*/, 0x9020_0000);
            v.context.set(ARG0, 0x0000_0000);
            v.context.set(ARG1, 0x0000_0000);
        }
    }
    //vm.run();

    Ok(())
}

/*
#[test_case]
fn test_vmem() -> Result<(), &'static str> {
    let mut vm: VirtualMachine = VirtualMachine::new(
        0,           /* CPUマスク */
        0x8020_0000, /* 開始アドレス(ジャンプ先) */
        0x9000_0000, /* ベースアドレス(物理メモリ) */
        0x1000_0000, /* メモリサイズ */
    );

    //vm.map_all_guest_page(); /* 登録されたページをすべてマップ(静的にマップするときに使う) */
    //vm.map_guest_page(guest_paddr); /* 指定ページをマップ(動的にマップするときに使う) */
    Ok(())
}*/
