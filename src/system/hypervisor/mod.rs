//! Hypervisor機能本体

pub mod mm;
use self::mm::*;

extern crate alloc;
use alloc::string::String;

extern crate core;
use core::intrinsics::transmute;

use crate::PERIPHERALS;

use crate::environment::traits::cpu::HasCpu;
use crate::environment::traits::intc::HasIntc;
use crate::environment::traits::timer::HasTimer;
use crate::driver::traits::cpu::TraitCpu;
use crate::driver::traits::intc::TraitIntc;
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

// 4KiBの仮想PLIC
#[repr(C)]
#[repr(align(4096))]
struct Vplic {
    reg: [u32; 1024]
}
extern crate register;
use register::{cpu::RegisterReadWrite/*, register_bitfields*/};

use csr::hvip::*;

pub fn do_ecall_from_vsmode(exc_num: usize, regs: &mut Registers)
{
    let mut ext: i32 = (*(regs)).gp[15] as i32;
    let mut fid: i32 = (*(regs)).gp[14] as i32;
    let mut a0: usize = (*(regs)).gp[8];
    let mut a1: usize = (*(regs)).gp[9];
    let mut a2: usize = (*(regs)).gp[10];
    let mut a3: usize = (*(regs)).gp[11];
    let mut a4: usize = (*(regs)).gp[12];
    let mut a5: usize = (*(regs)).gp[13];

    if (ext == 0) {
        let cpu = unsafe { PERIPHERALS.take_cpu() };

        //let sie = Sie{};
        //sie.modify(sie::STIE::SET);
        cpu.enable_interrupt_mask(Interrupt::SupervisorTimerInterrupt.mask());

        let hvip = Hvip{};
        hvip.modify(hvip::VSTIP::CLEAR);
        //cpu.flush_interrupt_pending();

        unsafe { PERIPHERALS.release_cpu(cpu) };
    } 
    
    if (ext == 6) {
 
        unsafe {
            if (VIRTUAL_PLIC.reg[81] == 0) {
                let cpu = unsafe { PERIPHERALS.take_cpu() };
                unsafe{map_vaddr(transmute(cpu.get_vs_pagetable()), transmute(&VIRTUAL_PLIC), 0xffffffd000201000);}
                //unsafe{map_vaddr(transmute(cpu.get_vs_pagetable()), transmute(&VIRTUAL_PLIC), 0xffffffd000201000);}
                unsafe { PERIPHERALS.release_cpu(cpu) };
                VIRTUAL_PLIC.reg[81] == 1;
            }
        }
        ext = 0x52464E43;
        fid = 6;
        a2 = a1;
        a3 = a2;
        a0 = 1;
        a1 = 0;
    }

    let ret = do_ecall(ext, fid, a0, a1, a2, a3 , a4, a5);

    (*(regs)).gp[8] = ret.0;
    (*(regs)).gp[9] = ret.1;

    (*(regs)).epc = (*(regs)).epc + 4;
}

pub fn do_load_page_fault(int_num: usize, regs: &mut Registers) {
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    //cpu.print_csr();   
    //println!("addr:{:x}", cpu.get_vs_fault_address());
    let fault_addr = cpu.get_vs_fault_address();
    if (fault_addr >= 0xffffffd000201000 && fault_addr < 0xffffffd000202000) {
        unsafe {
            (*(regs)).gp[8] = VIRTUAL_PLIC.reg[1] as usize;
            VIRTUAL_PLIC.reg[1] = 0;
            (*(regs)).epc = (*(regs)).epc + 4;
        }
    }
    else {
        redirect_to_guest(regs);
    }

    unsafe { PERIPHERALS.release_cpu(cpu) };
}

static mut VIRTUAL_PLIC: Vplic = Vplic{reg: [0 as u32; 1024]};

pub fn do_supervisor_external_interrupt(int_num: usize, regs: &mut Registers)
{
    let cpu = unsafe { PERIPHERALS.take_cpu() };

    let intc = unsafe { PERIPHERALS.take_intc() };
    // PLICでペンディングビットをクリア
    let int_id = intc.get_pend_int();

    //test
    // Linuxのページテーブルを書換え
    // キャッシュクリア前にページエントリを書換え
    unsafe {
        /*
        if (VIRTUAL_PLIC.reg[1] == 0) {
            let cpu = unsafe { PERIPHERALS.take_cpu() };
            unsafe{map_vaddr(transmute(cpu.get_vs_pagetable()), transmute(&VIRTUAL_PLIC), 0xffffffd000201000);}
            unsafe { PERIPHERALS.release_cpu(cpu) };
        }*/
        VIRTUAL_PLIC.reg[1] = int_id;
    }  

    /* 自分への割込みは無効に */
    //cpu.disable_interrupt_mask(Interrupt::SupervisorExternalInterrupt.mask());
    cpu.assert_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());
    //cpu.flush_interrupt_pending();
    //redirect_to_guest(regs);

    unsafe { PERIPHERALS.release_cpu(cpu) };
    
    // PLICでペンディングビットをクリア
    intc.set_comp_int(int_id);
    unsafe { PERIPHERALS.release_intc(intc) };
    //println!("ex int: {}", int_id); //printすると、割込みが入る？なるべく避ける
}

pub fn do_supervisor_timer_interrupt(int_num: usize, regs: &mut Registers)
{
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    /* ゲストにタイマ割込みをあげる */
    //cpu.assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
    /* 自分への割込みは無効に */
    
    cpu.disable_interrupt_mask(Interrupt::SupervisorTimerInterrupt.mask());
    cpu.assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());

    //redirect_to_guest(regs);
    //cpu.print_csr();
    unsafe { PERIPHERALS.release_cpu(cpu) };

}


pub fn init() {
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    
    cpu.switch_hs_mode();

    cpu.enable_interrupt();
    cpu.set_default_vector();

    
    cpu.register_interrupt(Interrupt::VirtualSupervisorTimerInterrupt as usize, echo_test);
    cpu.register_exception(Exception::LoadPageFault as usize, echo_test);
    
    cpu.register_interrupt(Interrupt::SupervisorSoftwareInterrupt as usize, echo_test);
    cpu.register_interrupt(5, do_supervisor_timer_interrupt);
    cpu.register_interrupt(6, echo_test);
    cpu.register_interrupt(9, do_supervisor_external_interrupt);
    cpu.register_exception(10, do_ecall_from_vsmode);
    cpu.register_exception(Exception::LoadPageFault as usize, do_load_page_fault);
    
    
    unsafe { PERIPHERALS.release_cpu(cpu) };

    //test
    //create_page_table();
    //enable_paging();
    
}

pub fn boot_guest() {
    let cpu = unsafe { PERIPHERALS.take_cpu() };

    // sret後に、VS-modeに移行させる
    cpu.set_next_mode(PrivilegeMode::ModeVS);
    //cpu.set_next_mode(PrivilegeMode::ModeS);
    
    //cpu.csr.hstatus.set(0x80 << 12);

    // 
    /*
    cpu.disable_interrupt_mask(
        //Interrupt::SupervisorSoftwareInterrupt.mask()// |
        //Interrupt::SupervisorTimerInterrupt.mask() |
        //Interrupt::SupervisorExternalInterrupt.mask() 
    );
    */
    cpu.enable_interrupt_mask(
        Interrupt::SupervisorSoftwareInterrupt.mask() |
        Interrupt::SupervisorTimerInterrupt.mask() |
        Interrupt::SupervisorExternalInterrupt.mask() |
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
        //Exception::LoadPageFault.mask() |
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
