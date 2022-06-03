//! Hypervisor機能本体

pub mod mm;
use self::mm::*;

pub mod virtdev;
use virtdev::vplic::VPlic;

extern crate alloc;
use alloc::string::String;

extern crate core;
use core::intrinsics::transmute;

use crate::CPU;
use crate::PERIPHERALS;

use crate::driver::traits::cpu::TraitCpu;
use crate::driver::traits::intc::TraitIntc;
use crate::driver::traits::serial::TraitSerial;
use crate::environment::traits::cpu::HasCpu;
use crate::environment::traits::intc::HasIntc;
use crate::environment::traits::timer::HasTimer;
use crate::environment::traits::serial::HasSerial;
//use crate::driver::traits::timer::TraitTimer;

use crate::driver::arch::rv64::*; /* todo delete*/

use crate::driver::traits::arch::riscv::Exception;
use crate::driver::traits::arch::riscv::Interrupt;
use crate::driver::traits::arch::riscv::PagingMode;
use crate::driver::traits::arch::riscv::PrivilegeMode;
use crate::driver::traits::arch::riscv::Registers;
use crate::driver::traits::arch::riscv::TraitRisvCpu;

use crate::library::vshell::{Command, VShell};

use crate::driver::arch::rv64::mmu::sv48::PageTableSv48;

use crate::print;
use crate::println;

fn echo_test(exc_num: usize, regs: &mut Registers) {
    println!("exceptioin occur!: {}", exc_num);
    CPU.print_csr();
}

static mut VPLIC: VPlic = VPlic::new();

pub fn do_ecall_from_vsmode(exc_num: usize, regs: &mut Registers) {
    let mut ext: i32 = (*(regs)).a7 as i32;
    let mut fid: i32 = (*(regs)).a6 as i32;
    let mut a0: usize = (*(regs)).a0;
    let mut a1: usize = (*(regs)).a1;
    let mut a2: usize = (*(regs)).a2;
    let mut a3: usize = (*(regs)).a3;
    let a4: usize = (*(regs)).a4;
    let a5: usize = (*(regs)).a5;

    /* タイマセット */
    if (ext == 0 || (fid == 0 && ext == 0x54494d45)) {
        unsafe {
            
            if (VPLIC.read32(81) == 0) {
                _map_vaddr48::<PageTableSv48>(
                    transmute(CPU.hyp.get_vs_pagetable()),
                    0xffff8f8000201000,
                );
                VPLIC.write32(81, 1);
            }
        }

        CPU.int.enable_mask_s(Interrupt::SupervisorTimerInterrupt.mask());
        CPU.hyp.flush_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());

    }
    /* キャッシュのフラッシュ */
    if (ext == 6) {
        
        ext = 0x52464E43;
        fid = 6;
        a2 = a1;
        a3 = a2;
        a0 = 1;
        a1 = 0;
        
    }

    let ret = CPU.inst.do_ecall(ext, fid, a0, a1, a2, a3, a4, a5);

    (*(regs)).a0 = ret.0;
    (*(regs)).a1 = ret.1;

    (*(regs)).epc = (*(regs)).epc + 4;
}

pub fn do_store_page_fault(int_num: usize, regs: &mut Registers) {
    let fault_addr = CPU.exc.get_fault_address();

    if (fault_addr >= 0xffff8f8000201000 && fault_addr < 0xffff8f8000202000) {
        if (fault_addr == 0xffff8f8000201004) {
            (*(regs)).epc = (*(regs)).epc + 2;
            CPU.hyp.flush_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());
        } else {
            println!("store addr:{:x}", fault_addr);
        }
    } else {
        redirect_to_guest(regs);
    }
}

pub fn do_load_page_fault(int_num: usize, regs: &mut Registers) {
    let fault_addr = CPU.exc.get_fault_address();
    if (fault_addr >= 0xffff8f8000201000 && fault_addr < 0xffff8f8000202000) {
        if (fault_addr == 0xffff8f8000201004) {
            unsafe {
                (*(regs)).a5 = VPLIC.read32(1) as usize;
                VPLIC.write32(1, 0);

                (*(regs)).epc = (*(regs)).epc + 4;
            }
        }
    } else {
        redirect_to_guest(regs);
    }

}

pub fn do_supervisor_external_interrupt(int_num: usize, regs: &mut Registers) {
    let intc = unsafe { PERIPHERALS.take_intc() };

    // 物理PLICからペンディングビットを読み、クリアする
    let int_id = intc.get_pend_int();

    unsafe {VPLIC.write32(1, int_id);}

    // 仮想外部割込みを発生させる
    CPU.hyp.assert_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());

    // PLICでペンディングビットをクリア
    intc.set_comp_int(int_id);

    unsafe { PERIPHERALS.release_intc(intc) };
}

pub fn do_supervisor_timer_interrupt(int_num: usize, regs: &mut Registers) {

    /* 自分への割込みは無効に */
    CPU.int.disable_mask_s(Interrupt::SupervisorTimerInterrupt.mask());
    /* ゲストにタイマ割込みをあげる */
    CPU.hyp.assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());

}

pub fn init() {
    CPU.switch_hs_mode();

    CPU.enable_interrupt();
    CPU.set_default_vector();

    CPU.register_interrupt(
        Interrupt::VirtualSupervisorTimerInterrupt as usize,
        echo_test,
    );
    CPU.register_exception(Exception::LoadPageFault as usize, echo_test);

    CPU.register_interrupt(Interrupt::SupervisorSoftwareInterrupt as usize, echo_test);
    CPU.register_interrupt(5, do_supervisor_timer_interrupt);
    CPU.register_interrupt(6, echo_test);
    CPU.register_interrupt(9, do_supervisor_external_interrupt);
    CPU.register_exception(10, do_ecall_from_vsmode);
    CPU.register_exception(Exception::LoadPageFault as usize, do_load_page_fault);
    CPU.register_exception(Exception::StoreAmoPageFault as usize, do_store_page_fault);

}

pub fn boot_guest() {

    // sret後に、VS-modeに移行させる
    CPU.set_next_mode(PrivilegeMode::ModeVS);
    //cpu.set_next_mode(PrivilegeMode::ModeS);

    /*
    cpu.disable_interrupt_mask(
        //Interrupt::SupervisorSoftwareInterrupt.mask()// |
        //Interrupt::SupervisorTimerInterrupt.mask() |
        //Interrupt::SupervisorExternalInterrupt.mask()
    );
    */
    CPU.int.enable_mask_s(
        Interrupt::SupervisorSoftwareInterrupt.mask()
            | Interrupt::SupervisorTimerInterrupt.mask()
            | Interrupt::SupervisorExternalInterrupt.mask()
            | Interrupt::VirtualSupervisorSoftwareInterrupt.mask()
            | Interrupt::VirtualSupervisorTimerInterrupt.mask()
            | Interrupt::VirtualSupervisorExternalInterrupt.mask()
            | Interrupt::SupervisorGuestExternalInterrupt.mask(),
    );

    //cpu.enable_external_interrupt_mask(0xffff_ffff);
    //CPU.hyp.enable_exint_mask_h(0xffff_ffff);

    CPU.hyp.set_delegation_exc(
        Exception::InstructionAddressMisaligned.mask()
            | Exception::Breakpoint.mask()
            | Exception::EnvironmentCallFromUmodeOrVUmode.mask()
            | Exception::InstructionPageFault.mask(), //|
                                                      //Exception::LoadPageFault.mask() |
                                                      //Exception::StoreAmoPageFault.mask()
    );
    
    CPU.hyp.set_delegation_int(        
        Interrupt::VirtualSupervisorSoftwareInterrupt.mask()
            | Interrupt::VirtualSupervisorTimerInterrupt.mask()
            | Interrupt::VirtualSupervisorExternalInterrupt.mask(),
    );

    CPU.hyp.flush_vsmode_interrupt(0xffff_ffff_ffff_ffff);

    CPU.mmu.set_paging_mode(PagingMode::Bare);

    CPU.hyp.enable_vsmode_counter_access(0xffff_ffff);

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

    pub fn run(&self) {
        init();
        println!("Hello I'm {} ", "Violet Hypervisor");

        //
        boot_guest();

        let mut vshell = VShell::new();
        vshell.add_cmd(Command {
            name: String::from("boot"),
            func: boot_guest,
        });
        vshell.run();
    }
}
