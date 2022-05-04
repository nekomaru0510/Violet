//! Hypervisor機能本体

extern crate alloc;
use alloc::string::String;

use crate::PERIPHERALS;

use crate::environment::traits::cpu::HasCpu;
use crate::environment::traits::timer::HasTimer;
use crate::driver::traits::cpu::TraitCpu;
//use crate::driver::traits::timer::TraitTimer;

use crate::driver::arch::rv64::*; /* todo delete*/

use crate::driver::traits::arch::riscv::TraitRisvCpu;
use crate::driver::traits::arch::riscv::Registers;
use crate::driver::traits::arch::riscv::PrivilegeMode;
use crate::driver::traits::arch::riscv::PagingMode;
use crate::driver::traits::arch::riscv::Exception;
use crate::driver::traits::arch::riscv::Interrupt;

use crate::library::vshell::{VShell, Command};

use crate::print;
use crate::println;

fn echo_test(exc_num: usize, regs: &mut Registers) {
    println!("exceptioin occur!: {}", exc_num);
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    //cpu.print_csr();
    //redirect_to_guest(regs);
    cpu.print_csr();
    unsafe { PERIPHERALS.release_cpu(cpu) };
}

pub fn do_supervisor_timer_interrupt(int_num: usize, regs: &mut Registers)
{
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    /* ゲストにタイマ割込みをあげる */
    //cpu.assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
    /* 自分への割込みは無効に */
    cpu.disable_interrupt_mask(Interrupt::SupervisorTimerInterrupt.mask());
    cpu.print_csr();
    cpu.assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
    cpu.print_csr();
    unsafe { PERIPHERALS.release_cpu(cpu) };
        
    //do_ecall(0, 0, 0x17b162, 0, 0, 0, 0, 0);
}


pub fn init() {
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    
    cpu.switch_hs_mode();

    cpu.enable_interrupt();
    cpu.set_default_vector();

    
    cpu.register_interrupt(Interrupt::VirtualSupervisorTimerInterrupt as usize, echo_test);
    cpu.register_exception(Exception::LoadPageFault as usize, echo_test);
    
    cpu.register_interrupt(5, do_supervisor_timer_interrupt);
    cpu.register_interrupt(6, echo_test);
    cpu.register_exception(10, do_ecall_from_vsmode);
    
    unsafe { PERIPHERALS.release_cpu(cpu) };
}

pub fn boot_guest() {
    let cpu = unsafe { PERIPHERALS.take_cpu() };

    // sret後に、VS-modeに移行させる
    cpu.set_next_mode(PrivilegeMode::ModeVS);
    //cpu.set_next_mode(PrivilegeMode::ModeS);
    
    //cpu.csr.hstatus.set(0x80 << 12);

    // 
    cpu.disable_interrupt_mask(
        Interrupt::SupervisorSoftwareInterrupt.mask() |
        //Interrupt::SupervisorTimerInterrupt.mask() |
        Interrupt::SupervisorExternalInterrupt.mask() 
    );
    cpu.enable_interrupt_mask(
        Interrupt::SupervisorTimerInterrupt.mask() |
        Interrupt::VirtualSupervisorSoftwareInterrupt.mask() |
        Interrupt::VirtualSupervisorTimerInterrupt.mask() |
        Interrupt::VirtualSupervisorExternalInterrupt.mask() |
        Interrupt::SupervisorGuestExternalInterrupt.mask()
    );

    cpu.enable_external_interrupt_mask(0xffff_ffff);

    cpu.enable_exception_delegation_mask(
        Exception::InstructionAddressMisaligned.mask() | 
        Exception::Breakpoint.mask() |
        Exception::EnvironmentCallFromUmodeOrVUmode.mask() |
        Exception::InstructionPageFault.mask() |
        Exception::LoadPageFault.mask() |
        Exception::StoreAmoPageFault.mask()
    );
    cpu.enable_interrupt_delegation_mask(
        Interrupt::VirtualSupervisorSoftwareInterrupt.mask() |
        Interrupt::VirtualSupervisorTimerInterrupt.mask() |
        Interrupt::VirtualSupervisorExternalInterrupt.mask()
    );

    cpu.flush_vsmode_interrupt();

    cpu.set_paging_mode(PagingMode::Bare);

    cpu.enable_vsmode_counter_access(0xffff_ffff);

    unsafe { PERIPHERALS.release_cpu(cpu) };
    jump_by_sret(0x8020_0000, 0, 0x8220_0000);
}

/* 一応、何らかの設定値を格納できるように */
pub struct Hypervisor {
    sched: i32,
}

impl Hypervisor {
    pub fn new() -> Hypervisor {
        Hypervisor{sched: 0, }
    }

    pub fn run(&self) {
        init();
        println!("Hello I'm {} ", "Violet Hypervisor");
        
        //
        boot_guest();
        //_jump_guest_kernel(0x8020_0000, 0, 0x8220_0000);
        
        let mut vshell = VShell::new();
        vshell.add_cmd(Command{name: String::from("boot"), func: boot_guest});
        vshell.run();
        
    }
}
