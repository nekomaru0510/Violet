//! Hypervisor機能本体

pub mod mm;
pub mod arch;
pub mod virtdev;

extern crate alloc;
use alloc::string::String;

use crate::CPU;

use crate::driver::traits::cpu::TraitCpu;

use crate::driver::traits::arch::riscv::Exception;
use crate::driver::traits::arch::riscv::Interrupt;
use crate::driver::traits::arch::riscv::PagingMode;
use crate::driver::traits::arch::riscv::PrivilegeMode;
use crate::driver::traits::arch::riscv::TraitRisvCpu;

use crate::library::vshell::{Command, VShell};

use mm::*;

use crate::print;
use crate::println;

pub fn setup_boot() {
    CPU.switch_hs_mode();

    CPU.enable_interrupt();
    CPU.set_default_vector();
    
    create_page_table();
    enable_paging();

    CPU.int.disable_mask_s(
        Interrupt::SupervisorSoftwareInterrupt.mask() |
        Interrupt::SupervisorTimerInterrupt.mask() |
        Interrupt::SupervisorExternalInterrupt.mask()
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
            | Exception::StoreAmoPageFault.mask()
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
    
    CPU.inst.jump_by_sret(0x8020_0000, 0, 0x8220_0000);
}

/* 一応、何らかの設定値を格納できるように */
pub struct Hypervisor {
    sched: i32,
}

impl Hypervisor {
    pub fn new() -> Hypervisor {
        Hypervisor { sched: 0 }
    }

    pub fn setup(&self) {
        println!("Hello I'm {} ", "Violet Hypervisor");
        
        /* ゲスト起動前のデフォルトセットアップ */
        setup_boot();

    }

    pub fn run(&self) {

        boot_guest();

        let mut vshell = VShell::new();
        vshell.add_cmd(Command {
            name: String::from("boot"),
            func: boot_guest,
        });
        vshell.run();
    }
}
