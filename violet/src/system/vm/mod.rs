//! VirtualMachine

pub mod mm;
pub mod vcpu;
pub mod vdev;
pub mod vmem;

extern crate alloc;
use alloc::boxed::Box;

use crate::CPU;

use crate::driver::arch::rv64::{Exception, Interrupt, PagingMode, PrivilegeMode, TraitRisvCpu};
use crate::driver::traits::cpu::TraitCpu;

use crate::driver::arch::rv64::mmu::sv48::PageTableSv48;
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

pub fn boot_guest() {
    /* sret後に、VS-modeに移行させるよう設定 */
    CPU.set_next_mode(PrivilegeMode::ModeVS);

    CPU.inst.jump_by_sret(0x8020_0000, 0, 0x8220_0000); //linux
                                                        //CPU.inst.jump_by_sret(0x9000_0000, 0, 0x8220_0000); //xv6
                                                        //CPU.inst.jump_by_sret(0x8000_0000, 0, 0x8220_0000); //xv6
}

pub const NUM_OF_CPUS: usize = 2;
pub const NUM_OF_ARGS: usize = 2;

#[derive(Clone, Copy)]
pub struct BootParam {
    addr: usize,
    arg: [usize; NUM_OF_ARGS],
}

impl BootParam {
    pub const fn new(start_addr: usize) -> Self {
        BootParam {
            addr: start_addr,
            arg: [0; NUM_OF_ARGS],
        }
    }

    pub fn set_addr(&mut self, addr: usize) {
        self.addr = addr;
    }

    pub fn get_addr(&self) -> usize {
        self.addr
    }

    pub fn set_arg(&mut self, arg: [usize; NUM_OF_ARGS]) {
        for i in 0..NUM_OF_ARGS {
            self.arg[i] = arg[i];
        }
    }

    pub fn get_arg(&self, arg_idx: usize) -> usize {
        self.arg[arg_idx]
    }
}

use vdev::VirtualDevice;
use vdev::VirtualIoMap;
use vmem::VirtualMemoryMap;

pub struct VirtualMachine {
    /* == 必須設定項目 == */
    cpu_mask: u64,
    //start_addr: usize, /* VM内の開始アドレス */
    param: [BootParam; NUM_OF_CPUS], /* コアごとのブート情報 */
    mem_start: usize,
    mem_size: usize,
    /* ================= */
    vmem_start: usize,
    vmem: VirtualMemoryMap,
    viomap: Option<VirtualIoMap>,
}

impl VirtualMachine {
    pub const fn new(
        cpu_mask: u64,
        start_addr: usize,
        mem_start: usize,
        mem_size: usize,
    ) -> VirtualMachine {
        VirtualMachine {
            cpu_mask,
            param: [BootParam::new(start_addr); NUM_OF_CPUS],
            mem_start,
            mem_size,
            vmem_start: 0,
            //vcpu: Vec::new(),
            vmem: VirtualMemoryMap::new(),
            viomap: None,
        }
    }

    pub fn setup(&self) {
        /* ゲスト起動前のデフォルトセットアップ */
        setup_boot();
    }

    pub fn run(&self) {
        boot_guest();
    }

    pub fn boot(&self, cpu_id: usize) {
        /* sret後に、VS-modeに移行させるよう設定 */
        CPU.set_next_mode(PrivilegeMode::ModeVS);
        CPU.inst.jump_by_sret(
            self.param[cpu_id].get_addr(),
            self.param[cpu_id].get_arg(0),
            self.param[cpu_id].get_arg(1),
        );
    }

    pub fn set_cpu(&mut self, cpu_mask: u64) {
        self.cpu_mask |= cpu_mask;
    }

    pub fn set_memory(&mut self, mem_start: usize, mem_size: usize) {
        self.mem_start = mem_start;
        self.mem_size = mem_size;
    }

    pub fn set_start_addr(&mut self, cpu_id: usize, start_addr: usize) {
        self.param[cpu_id].set_addr(start_addr);
    }

    pub fn set_boot_arg(&mut self, cpu_id: usize, boot_arg: [usize; NUM_OF_ARGS]) {
        self.param[cpu_id].set_arg(boot_arg);
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
    let mut vm: VirtualMachine = VirtualMachine::new(
        0,           /* CPUマスク */
        0x8020_0000, /* 開始アドレス(ジャンプ先) */
        0x9000_0000, /* ベースアドレス(物理メモリ) */
        0x1000_0000, /* メモリサイズ */
    );
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
