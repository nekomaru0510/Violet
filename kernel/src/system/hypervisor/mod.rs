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
    let mut ext: i32 = (*(regs)).a7 as i32;
    let mut fid: i32 = (*(regs)).a6 as i32;
    let mut a0: usize = (*(regs)).a0;
    let mut a1: usize = (*(regs)).a1;
    let mut a2: usize = (*(regs)).a2;
    let mut a3: usize = (*(regs)).a3;
    let mut a4: usize = (*(regs)).a4;
    let mut a5: usize = (*(regs)).a5;

    /* タイマセット */
    if (ext == 0 || (fid == 0 && ext == 0x54494d45)) {
        unsafe {
            if (VIRTUAL_PLIC.reg[81] == 0) {
                let cpu = unsafe { PERIPHERALS.take_cpu() };
                //unsafe{map_vaddr(transmute(cpu.get_vs_pagetable()), transmute(&VIRTUAL_PLIC), 0xffffffd000201000);}
                unsafe{map_vaddr48(transmute(cpu.get_vs_pagetable()), transmute(&VIRTUAL_PLIC), 0xffff8f8000201000);}
                unsafe { PERIPHERALS.release_cpu(cpu) };
                VIRTUAL_PLIC.reg[81] == 1;
            }
        }

        let cpu = unsafe { PERIPHERALS.take_cpu() };

        cpu.enable_interrupt_mask(Interrupt::SupervisorTimerInterrupt.mask());

        let hvip = Hvip{};
        hvip.modify(hvip::VSTIP::CLEAR);
        //cpu.flush_interrupt_pending();

        unsafe { PERIPHERALS.release_cpu(cpu) };
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

    let ret = do_ecall(ext, fid, a0, a1, a2, a3 , a4, a5);

    (*(regs)).a0 = ret.0;
    (*(regs)).a1 = ret.1;

    (*(regs)).epc = (*(regs)).epc + 4;

}

pub fn do_store_page_fault(int_num: usize, regs: &mut Registers) {
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    let fault_addr = cpu.get_fault_address();

    if (fault_addr >= 0xffff8f8000201000 && fault_addr < 0xffff8f8000202000) {
        if (fault_addr == 0xffff8f8000201004) {
            //println!("store addr:{:x}", fault_addr);
            (*(regs)).epc = (*(regs)).epc + 2; //0xffffffe000270006
            //(*(regs)).epc = (*(regs)).epc + 4;
            /*
            let intc = unsafe { PERIPHERALS.take_intc() };
            intc.set_comp_int(10);
            unsafe { PERIPHERALS.release_intc(intc) };
             */
            let hvip = Hvip{};
            hvip.modify(hvip::VSEIP::CLEAR);
        }
        else {
            println!("store addr:{:x}", fault_addr);
        }
    }
    else {
        redirect_to_guest(regs);
    }
    unsafe { PERIPHERALS.release_cpu(cpu) };
}

pub fn do_load_page_fault(int_num: usize, regs: &mut Registers) {
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    //cpu.print_csr();   
    //println!("addr:{:x}", cpu.get_vs_fault_address());
    let fault_addr = cpu.get_fault_address();
    if (fault_addr >= 0xffff8f8000201000 && fault_addr < 0xffff8f8000202000) {
        //println!("load addr:{:x}", fault_addr);
        if (fault_addr == 0xffff8f8000201004) {
            unsafe {
                if (VIRTUAL_PLIC.reg[1] == 0) {
                    //cpu.print_csr();   
                }
            }
            unsafe {
                (*(regs)).a5 = VIRTUAL_PLIC.reg[1] as usize;
                NUM_OF_INT = NUM_OF_INT - 1;
                if (NUM_OF_INT >= 1) {

                }
                else if (NUM_OF_INT == 0) {
                    VIRTUAL_PLIC.reg[1] = 0;
                }
                else if (NUM_OF_INT < 0) {
                    NUM_OF_INT = 0;
                    //cpu.print_csr(); 
                    //println!("???");
                }

                (*(regs)).epc = (*(regs)).epc + 4;
            }
        }
        else {
            println!("load addr:{:x}", fault_addr);
        }
    }
    else {
        redirect_to_guest(regs);
        if (cpu.get_vs_fault_address() == 0xffff8f8000201004) {
            println!("???");
        }
    }

    unsafe { PERIPHERALS.release_cpu(cpu) };
}

static mut VIRTUAL_PLIC: Vplic = Vplic{reg: [0 as u32; 1024]};
static mut NUM_OF_INT: isize = 0;

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
        if (int_id != 10) {
            println!("int:{}", int_id);
        }
        VIRTUAL_PLIC.reg[1] = int_id;
        NUM_OF_INT = NUM_OF_INT + 1;
        //println!("int:{}", int_id);
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
    cpu.register_exception(Exception::StoreAmoPageFault as usize, do_store_page_fault);
    
    unsafe { PERIPHERALS.release_cpu(cpu) };

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

    //cpu.enable_external_interrupt_mask(0xffff_ffff);

    cpu.enable_exception_delegation_mask(
        Exception::InstructionAddressMisaligned.mask() | 
        Exception::Breakpoint.mask() |
        Exception::EnvironmentCallFromUmodeOrVUmode.mask() |
        Exception::InstructionPageFault.mask() //|
        //Exception::LoadPageFault.mask() |
        //Exception::StoreAmoPageFault.mask()
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

const NUM_OF_INIT_CALLS: usize = 8;
pub static mut INIT_CALLS: [Option<fn(regs: &mut Registers)>; NUM_OF_INIT_CALLS] = [None; NUM_OF_INIT_CALLS];
// [todo fix] 登録用マクロを用意する


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
        
        let mut vshell = VShell::new();
        vshell.add_cmd(Command{name: String::from("boot"), func: boot_guest});
        vshell.run();
        
    }
}
