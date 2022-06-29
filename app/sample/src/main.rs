//! Violetアプリケーションのサンプル(Linuxカーネルを動作させる)
#![no_main]
#![no_std]

extern crate violet;

use violet::CPU;
use violet::PERIPHERALS;

use violet::system::hypervisor::virtdev::vplic::VPlic;
use violet::system::hypervisor::arch::riscv::sbi;
use violet::system::hypervisor::mm::*;

use violet::driver::traits::arch::riscv::{Exception, Interrupt, Registers, TraitRisvCpu};
use violet::driver::arch::rv64::mmu::sv48::PageTableSv48;
use violet::driver::arch::rv64::redirect_to_guest;

use violet::environment::traits::intc::HasIntc;
use violet::driver::traits::intc::TraitIntc;

use crate::violet::driver::traits::cpu::mmu::PageTable; // test
use crate::violet::driver::traits::cpu::mmu::PageEntry; //test

extern crate core;
use core::intrinsics::transmute;

#[link_section = ".init_calls"]
#[no_mangle]
pub static mut INIT_CALLS: Option<fn()> = Some(init_sample);

static mut VPLIC: VPlic = VPlic::new();
static mut IS_FIRST: u32 = 0;

pub fn do_ecall_from_vsmode(regs: &mut Registers) {
    let mut ext: i32 = (*(regs)).a7 as i32;
    let mut fid: i32 = (*(regs)).a6 as i32;
    let mut a0: usize = (*(regs)).a0;
    let mut a1: usize = (*(regs)).a1;
    let mut a2: usize = (*(regs)).a2;
    let mut a3: usize = (*(regs)).a3;
    let a4: usize = (*(regs)).a4;
    let a5: usize = (*(regs)).a5;

    /* タイマセット */
    if (ext == sbi::Extension::SetTimer as i32 || ext == sbi::Extension::Timer as i32) {
        unsafe {
            if (IS_FIRST == 0) {
                invalid_page::<PageTableSv48>(
                    transmute(CPU.hyp.get_vs_pagetable()),
                    0xffff8f8000201000,
                );
                IS_FIRST = 1;
            }
        }
        /* 仮想タイマ割込みのフラッシュ */
        /* QEMU virtの性質上、ゲストOSは必ずタイマ割込みハンドラ内でタイマセットを行う */
        /* そのため、ここで仮想タイマ割込みのフラッシュを行う */
        CPU.hyp.flush_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
    }
    /* キャッシュのフラッシュ */
    if (ext == sbi::Extension::RemoteSfenceVma as i32) {
        
        ext = sbi::Extension::Rfence as i32;
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

pub fn do_guest_store_page_fault(regs: &mut Registers) {
    let paddr = CPU.hyp.get_vs_fault_paddr() as usize;
    map_vaddr::<PageTableSv48>(unsafe {transmute(CPU.hyp.get_hs_pagetable()) }, paddr, paddr);
}

pub fn do_guest_load_page_fault(regs: &mut Registers) {
    let paddr = CPU.hyp.get_vs_fault_paddr() as usize;
    map_vaddr::<PageTableSv48>(unsafe {transmute(CPU.hyp.get_hs_pagetable()) }, paddr, paddr);
}

pub fn do_guest_instruction_page_fault(regs: &mut Registers) {
    let paddr = CPU.hyp.get_vs_fault_paddr() as usize;
    map_vaddr::<PageTableSv48>(unsafe {transmute(CPU.hyp.get_hs_pagetable()) }, paddr, paddr);
}

pub fn do_store_page_fault(regs: &mut Registers) {
    /* 例外が発生したアドレスを物理アドレスに変換 */
    let fault_paddr = to_paddr::<PageTableSv48>(
        unsafe {transmute(CPU.hyp.get_vs_pagetable()) } ,
        CPU.exc.get_fault_address() as usize,
    );

    if (0x0c20_1000 <= fault_paddr && fault_paddr < 0x0c20_1000 + 0x1000) {        
        (*(regs)).epc = (*(regs)).epc + 2;
        CPU.hyp.flush_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());
    } else {
        redirect_to_guest(regs);
    }
}

pub fn do_load_page_fault(regs: &mut Registers) {
    /* 例外が発生したアドレスを物理アドレスに変換 */
    let fault_paddr = to_paddr::<PageTableSv48>(
        unsafe {transmute(CPU.hyp.get_vs_pagetable()) } ,
        CPU.exc.get_fault_address() as usize,
    );

    if (0x0c20_1000 <= fault_paddr && fault_paddr < 0x0c20_1000 + 0x1000) {
        unsafe {
            (*(regs)).a5 = VPLIC.read32(fault_paddr & 0x0000_1fff) as usize;
            (*(regs)).epc = (*(regs)).epc + 4; /* [todo fix] ld/sdの命令を解釈して、4byteか2byteか決めるべき */
        }
    } else {
        redirect_to_guest(regs);
    }
}

pub fn do_supervisor_external_interrupt(_regs: &mut Registers) {
    let intc = unsafe { PERIPHERALS.take_intc() };

    // 物理PLICからペンディングビットを読み、クリアする
    let int_id = intc.get_pend_int();

    // 仮想PLICへ書込み
    unsafe {VPLIC.write32(0x1004, int_id);}

    // 仮想外部割込みを発生させる
    CPU.hyp.assert_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());

    // PLICでペンディングビットをクリア
    intc.set_comp_int(int_id);

    unsafe { PERIPHERALS.release_intc(intc) };
}

pub fn do_supervisor_timer_interrupt(_regs: &mut Registers) {

    /* タイマの無効化 */
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    /* ゲストにタイマ割込みをあげる */
    CPU.hyp.assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
}

pub fn init_sample() {
    /* 割込みを有効化 */
    CPU.int.enable_mask_s(
        Interrupt::SupervisorTimerInterrupt.mask() |
        Interrupt::SupervisorExternalInterrupt.mask()
    );
    
    /* ゲストOSへの例外移譲を解除 */
    CPU.hyp.clear_delegation_exc(
        Exception::LoadPageFault.mask() 
        | Exception::StoreAmoPageFault.mask()
    );

    /* 割込みハンドラの登録 */
    CPU.register_interrupt(Interrupt::SupervisorTimerInterrupt, do_supervisor_timer_interrupt);
    CPU.register_interrupt(Interrupt::SupervisorExternalInterrupt, do_supervisor_external_interrupt);
    
    /* 例外ハンドラの登録 */
    CPU.register_exception(Exception::EnvironmentCallFromVSmode, do_ecall_from_vsmode);
    CPU.register_exception(Exception::LoadPageFault, do_load_page_fault);
    CPU.register_exception(Exception::StoreAmoPageFault, do_store_page_fault);

    CPU.register_exception(Exception::LoadGuestPageFault, do_guest_load_page_fault);
    CPU.register_exception(Exception::StoreAmoGuestPageFault, do_guest_store_page_fault);
    CPU.register_exception(Exception::InstructionGuestPageFault, do_guest_instruction_page_fault);
    
    
}
