//! Violetアプリケーションのサンプル(Linuxカーネルを動作させる)
#![no_main]
#![no_std]

extern crate violet;

use violet::environment::cpu_mut;
use violet::CPU;

use violet::system::vm::vdev::vplic::VPlic;
use violet::system::vm::VirtualMachine;

use violet::driver::arch::rv64::instruction::load::Load;
use violet::driver::arch::rv64::instruction::store::Store;
use violet::driver::arch::rv64::instruction::*;
use violet::driver::arch::rv64::regs::*;
use violet::driver::arch::rv64::sbi;
use violet::driver::arch::rv64::{Exception, Interrupt};

use violet::driver::arch::rv64::vscontext::*;
use violet::driver::traits::cpu::context::TraitContext;

use violet::environment::current_container; /* [todo delete] */
use violet::kernel::syscall::vsi::create_task;

use violet::app_init;
app_init!(sample_main);

static mut VM: VirtualMachine = VirtualMachine::new();

pub fn do_ecall_from_vsmode(regs: &mut Registers) {
    let ext: i32 = regs.reg[A7] as i32;
    let fid: i32 = regs.reg[A6] as i32;

    match sbi::Extension::from_ext(ext) {
        /* タイマセット */
        sbi::Extension::SetTimer | sbi::Extension::Timer => {
            CPU.hyp
                .flush_vsmode_interrupt(Interrupt::VirtualSupervisorTimerInterrupt.mask());
        }
        sbi::Extension::HartStateManagement => {
            if fid == 0 {
                regs.reg[A0] = 0;
                regs.reg[A1] = 0;
                regs.epc = regs.epc + 4;

                return;
            }
        }
        sbi::Extension::SystemReset => loop {},
        _ => {}
    }

    let ret = Instruction::ecall(
        ext,
        fid,
        regs.reg[A0],
        regs.reg[A1],
        regs.reg[A2],
        regs.reg[A3],
        regs.reg[A4],
        regs.reg[A5],
    );

    regs.reg[A0] = ret.0;
    regs.reg[A1] = ret.1;

    regs.epc = regs.epc + 4; /* todo fix */
}

/* [todo delete] */
fn topaddr(epc: usize) -> usize {
    (epc & 0x0_ffff_ffff) + 0x1000_0000 + 0x20_0000
}

pub fn do_guest_store_page_fault(regs: &mut Registers) {
    let fault_paddr = CPU.hyp.get_vs_fault_paddr() as usize;

    let inst = Instruction::fetch(topaddr(regs.epc));
    let val = Store::from_val(inst).store_value(regs);

    match unsafe { VM.write_dev(fault_paddr, val) } {
        None => unsafe {
            VM.map_guest_page(CPU.hyp.get_vs_fault_paddr() as usize);
        },
        Some(()) => {
            regs.epc = regs.epc + Instruction::len(inst);
            CPU.hyp
                .flush_vsmode_interrupt(Interrupt::VirtualSupervisorExternalInterrupt.mask());
        }
    }
}

pub fn do_guest_load_page_fault(regs: &mut Registers) {
    let fault_paddr = CPU.hyp.get_vs_fault_paddr() as usize;
    //let inst = fetch_inst(regs.epc);
    let inst = Instruction::fetch(topaddr(regs.epc));

    match unsafe { VM.read_dev(fault_paddr) } {
        None => unsafe {
            VM.map_guest_page(CPU.hyp.get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()/*get_load_reg(inst)*/] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    }
}

pub fn do_guest_instruction_page_fault(_regs: &mut Registers) {
    unsafe {
        VM.map_guest_page(CPU.hyp.get_vs_fault_paddr() as usize);
    }
}

pub fn do_supervisor_external_interrupt(_regs: &mut Registers) {
    let con = current_container(); /* [todo delete] アプリがコンテナを意識するのはおかしい */

    // 物理PLICからペンディングビットを読み、クリアする
    let int_id = match &con.unwrap().intc {
        None => 0,
        Some(i) => i.get_pend_int(),
    };

    // 仮想PLICへ書込み
    unsafe {
        match VM.get_dev_mut(0x0c20_1000) {
            // [todo fix] 割込み番号で検索できるようにする
            None => (),
            Some(d) => {
                d.interrupt(int_id as usize);
            }
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

pub fn boot_linux() {
    unsafe {
        VM.setup();
    }

    /* 割込みを有効化 */
    CPU.int.enable_mask_s(
        Interrupt::SupervisorTimerInterrupt.mask() | Interrupt::SupervisorExternalInterrupt.mask(),
    );

    /* 割込みハンドラの登録 */
    cpu_mut().register_interrupt(
        Interrupt::SupervisorTimerInterrupt,
        do_supervisor_timer_interrupt,
    );
    cpu_mut().register_interrupt(
        Interrupt::SupervisorExternalInterrupt,
        do_supervisor_external_interrupt,
    );

    /* 例外ハンドラの登録 */
    cpu_mut().register_exception(Exception::EnvironmentCallFromVSmode, do_ecall_from_vsmode);
    cpu_mut().register_exception(Exception::LoadGuestPageFault, do_guest_load_page_fault);
    cpu_mut().register_exception(Exception::StoreAmoGuestPageFault, do_guest_store_page_fault);
    cpu_mut().register_exception(
        Exception::InstructionGuestPageFault,
        do_guest_instruction_page_fault,
    );

    unsafe {
        VM.run();
    }
}

pub fn sample_main() {
    let boot_core = 1;
    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([boot_core, 0]); /* vcpu0 ... pcpu1 */
    unsafe {
        /* CPU */
        VM.register_cpu(0, boot_core); /* vcpu0 ... pcpu1 */
        match VM.vcpu_mut(0) {
            None => (),
            Some(v) => {
                v.context.set(JUMP_ADDR, 0x8020_0000);
                v.context.set(ARG0, 0);
                v.context.set(ARG1, 0x8220_0000);
            }
        }
        /* RAM */
        VM.register_mem(0x8020_0000, 0x9020_0000, 0x1000_0000);
        VM.register_mem(0x8220_0000, 0x8220_0000, 0x2_0000); //FDTは物理メモリにマップ サイズは適当
        VM.register_mem(0x8810_0000, 0x88100000, 0x20_0000); //initrdも物理メモリにマップ サイズはrootfs.imgより概算
                                                             /* MMIO */
        VM.register_dev(0x0c00_0000, 0x0400_0000, vplic);
    }

    /* コア1でLinuxを起動させる */
    create_task(2, boot_linux, boot_core);
}
