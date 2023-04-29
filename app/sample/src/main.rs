//! Violetアプリケーションのサンプル(Linuxカーネルを動作させる)
#![no_main]
#![no_std]

extern crate violet;

use violet::CPU;

use violet::driver::arch::rv64::inst;
use violet::driver::arch::rv64::inst::*;
use violet::driver::arch::rv64::regs::Registers;
use violet::driver::arch::rv64::regs::*;

use violet::system::vm::VirtualMachine;
use violet::system::vm::mm::*;
use violet::system::vm::vdev::vplic::VPlic;
use violet::system::vm::vdev::VirtualDevice;

use violet::driver::arch::rv64::mmu::sv48::PageTableSv48;
use violet::driver::arch::rv64::sbi;
use violet::driver::traits::arch::riscv::{Exception, Interrupt, TraitRisvCpu};
use violet::driver::traits::intc::TraitIntc;

use violet::kernel::syscall::toppers::{T_CTSK, cre_tsk};
use violet::kernel::container::*;

use crate::violet::library::std::memcpy;

extern crate core;
use core::intrinsics::transmute;

use violet::app_init;
app_init!(sample_main);

static mut VM: VirtualMachine = VirtualMachine::new(
    0,              /* CPUマスク */
    0x8020_0000,    /* 開始アドレス(ジャンプ先) */
    0x9000_0000,    /* ベースアドレス(物理メモリ) */
    0x1000_0000     /* メモリサイズ */
);

pub fn do_ecall_from_vsmode(regs: &mut Registers) {
    let mut ext: i32 = regs.reg[A7] as i32;
    let mut fid: i32 = regs.reg[A6] as i32;
    let mut a0: usize = regs.reg[A0];
    let mut a1: usize = regs.reg[A1];
    let mut a2: usize = regs.reg[A2];
    let mut a3: usize = regs.reg[A3];
    let a4: usize = regs.reg[A4];
    let a5: usize = regs.reg[A5];

    /* タイマセット */
    if (ext == sbi::Extension::SetTimer as i32 || ext == sbi::Extension::Timer as i32) {
        /* 仮想タイマ割込みのフラッシュ */
        /* QEMU virtの性質上、ゲストOSは必ずタイマ割込みハンドラ内でタイマセットを行う */
        /* そのため、ここで仮想タイマ割込みのフラッシュを行う */
        CPU.hyp
            .flush_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
    }
    /* キャッシュのフラッシュ */
    if (ext == sbi::Extension::RemoteSfenceVma as i32) {
        ext = sbi::Extension::Rfence as i32;
        fid = 6;
        a2 = a1;
        a3 = a2;
        a0 = a0 + 0x1000_0000;
        a1 = 0;
    }
    /* CPUのキック */
    if (ext == sbi::Extension::HartStateManagement as i32) {
        if (fid == 0) {
            unsafe{
                VM.set_start_addr(a0, a1);
                VM.set_boot_arg(a0, [a2, a2]);
            } 
            
            cre_tsk(1+a0, &T_CTSK{task:secondary_boot, prcid:a0});
            
            regs.reg[A0] = 0;
            regs.reg[A1] = 0;
            regs.epc = regs.epc + 4;
            
            /* 2コア目以降のキック (現状、起床させても正常に動かない) */
            //let hart_mask: u64 = 0x01 << a0;
            //sbi::sbi_send_ipi(&hart_mask);
            return;
        }
    }
    /* システムのリセット */
    if (ext == sbi::Extension::SystemReset as i32) {
        loop{}
    }

    let ret = CPU.inst.do_ecall(ext, fid, a0, a1, a2, a3, a4, a5);
    /*
    let con = get_container(current_container());
    let int_id = match &con.unwrap().cpu[0] {
        None => 0,
        Some(c) => c.inst.do_ecall(ext, fid, a0, a1, a2, a3, a4, a5),
    };
    */

    regs.reg[A0] = ret.0;
    regs.reg[A1] = ret.1;

    regs.epc = regs.epc + 4;
}

pub fn get_real_paddr(guest_paddr: usize) -> usize {
    if (guest_paddr < 0x8000_0000) {
        guest_paddr
    } else {
        guest_paddr + 0x1000_0000
    }
}

pub fn map_guest_page() {
    let gpaddr = CPU.hyp.get_vs_fault_paddr() as usize;
    let paddr = get_real_paddr(gpaddr);
    //let paddr = gpaddr;
    map_vaddr::<PageTableSv48>(
        unsafe { transmute(CPU.hyp.get_hs_pagetable()) },
        paddr,
        gpaddr,
    );
}

/* [todo delete] */
use violet::system::vm::vdev::read_raw;
fn fetch_inst(epc: usize) -> usize {
    read_raw((epc & 0x0_ffff_ffff) + 0x1000_0000 + 0x20_0000)
}

extern crate alloc;
use alloc::vec::Vec;

pub fn do_guest_store_page_fault(regs: &mut Registers) {
    let fault_paddr = CPU.hyp.get_vs_fault_paddr() as usize;
    
    let inst = fetch_inst(regs.epc);
    let val = get_store_value(inst, regs);

    if (0x0c00_0000 <= fault_paddr && fault_paddr < 0x0c20_1000 + 0x1000) {
        unsafe {
            match VM.get_dev_mut::<VPlic>(fault_paddr) {
                None => (),
                Some(d) => {
                    d.write32(fault_paddr, val as u32);
                },
            }
        }
        
        if (is_compressed(inst)) {
            regs.epc = regs.epc + 2;
        }
        else {
            regs.epc = regs.epc + 4;
        }

        CPU.hyp
            .flush_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());
    } else {
        map_guest_page();
    }
}

pub fn do_guest_load_page_fault(regs: &mut Registers) {
    let fault_paddr = CPU.hyp.get_vs_fault_paddr() as usize;
    let inst = fetch_inst(regs.epc);

    if (0x0c00_0000 <= fault_paddr && fault_paddr < 0x0c20_1000 + 0x1000) {        
    //if (0x0c20_1000 <= fault_paddr && fault_paddr < 0x0c20_1000 + 0x1000) {        
        unsafe {
            let reg_idx = get_load_reg(inst);
            regs.reg[reg_idx] = match VM.get_dev_mut::<VPlic>(fault_paddr) {
                None => regs.reg[reg_idx],
                Some(d) => {
                    d.read32(fault_paddr) as usize
                },
            };
            
            if (is_compressed(inst)) {
                regs.epc = regs.epc + 2;
            }
            else {
                regs.epc = regs.epc + 4;
            }
        }
    } else {
        map_guest_page();
    }
}

pub fn do_guest_instruction_page_fault(regs: &mut Registers) {
    map_guest_page();
}

pub fn do_supervisor_external_interrupt(regs: &mut Registers) {

    let con = current_container();

    // 物理PLICからペンディングビットを読み、クリアする
    let int_id = match &con.unwrap().intc {
        None => 0,
        Some(i) => i.get_pend_int(),
    };

    // 仮想PLICへ書込み
    unsafe {
        match VM.get_dev_mut::<VPlic>(0x0c20_1000) {
            None => (),
            Some(d) => {
                d.interrupt(int_id as usize);//write32(0x0C20_1004, int_id);
            },
        }
    }

    // 仮想外部割込みを発生させる
    CPU.hyp
        .assert_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());

    // PLICでペンディングビットをクリア
    match &con.unwrap().intc {
        None => (),
        Some(i) => i.set_comp_int(int_id),
    }

}

pub fn do_supervisor_timer_interrupt(_regs: &mut Registers) {
    /* タイマの無効化 */
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    /* ゲストにタイマ割込みをあげる */
    CPU.hyp
        .assert_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
}

pub fn setup_cpu() {

    /* 割込みを有効化 */
    CPU.int.enable_mask_s(
        Interrupt::SupervisorTimerInterrupt.mask() | Interrupt::SupervisorExternalInterrupt.mask(),
    );

    /* 割込みハンドラの登録 */
    CPU.register_interrupt(
        Interrupt::SupervisorTimerInterrupt,
        do_supervisor_timer_interrupt,
    );
    CPU.register_interrupt(
        Interrupt::SupervisorExternalInterrupt,
        do_supervisor_external_interrupt,
    );

    /* 例外ハンドラの登録 */
    CPU.register_exception(Exception::EnvironmentCallFromVSmode, do_ecall_from_vsmode);
    CPU.register_exception(Exception::LoadGuestPageFault, do_guest_load_page_fault);
    CPU.register_exception(Exception::StoreAmoGuestPageFault, do_guest_store_page_fault);
    CPU.register_exception(
        Exception::InstructionGuestPageFault,
        do_guest_instruction_page_fault,
    );
}

pub fn boot_linux() {
    let cpu_id: usize = 0;

    /* [todo fix] コピーではなく、メモリマップに変更する */
    memcpy(0x8220_0000 + 0x1000_0000, 0x8220_0000, 0x2_0000); //FDT サイズは適当
    memcpy(0x88100000 + 0x1000_0000, 0x88100000, 0x20_0000); //initrd サイズはrootfs.imgより概算

    unsafe {
        VM.setup();
    }

    setup_cpu();

    unsafe {
        VM.set_start_addr(cpu_id, 0x8020_0000);
        VM.set_boot_arg(cpu_id, [0, 0x8220_0000]);
        VM.boot(cpu_id);
    }
}

pub fn secondary_boot() {
    unsafe {
        VM.setup();
    }

    setup_cpu();
 
    unsafe {
        VM.boot(1)
    }
}

pub fn sample_main() {

    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([1, 1]);
    unsafe{ VM.register_dev(0x0c00_0000, 0x0400_0000, vplic); }
    //unsafe{ VM.register_dev(0x0c20_1000, 0x1000, vplic); }

    boot_linux();

    loop{}
}


