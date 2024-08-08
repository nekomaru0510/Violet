//! Violetアプリケーションのサンプル(Linuxカーネルを動作させる)
#![no_main]
#![no_std]

extern crate violet;

use violet::environment::cpu_mut;

use violet::library::vm::vdev::vplic::VPlic;
use violet::library::vm::vdev::vclint::VClint;
use violet::library::vm::VirtualMachine;

use violet::driver::arch::rv64::extension::hypervisor::Hext;
use violet::driver::arch::rv64::instruction::load::Load;
use violet::driver::arch::rv64::instruction::store::Store;
use violet::driver::arch::rv64::instruction::*;
use violet::driver::arch::rv64::regs::*;
use violet::driver::arch::rv64::sbi;
use violet::driver::arch::rv64::trap::int::Interrupt;
use violet::driver::arch::rv64::trap::TrapVector;
use violet::driver::arch::rv64::vscontext::*;
use violet::driver::traits::cpu::context::TraitContext;

use violet::kernel::syscall::vsi::create_task;
use violet::resource::{get_resources, BorrowResource, ResourceType};

use violet::app_init;
app_init!(sample_main);

static mut VM: VirtualMachine<Hext> = VirtualMachine::new();

pub fn do_ecall_from_vsmode(sp: *mut usize /*regs: &mut Registers*/) {
    let regs = Registers::from(sp);
    let ext: i32 = regs.reg[A7] as i32;
    let fid: i32 = regs.reg[A6] as i32;

    match sbi::Extension::from_ext(ext) {
        /* タイマセット */
        sbi::Extension::SetTimer | sbi::Extension::Timer => {
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
            ));
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
    (epc & 0x0_ffff_ffff) + 0x1000_0000 + 0x20_0000 //linux
}

pub fn do_guest_store_page_fault(sp: *mut usize /*regs: &mut Registers*/) {
    let regs = Registers::from(sp);
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;

    let inst = Instruction::fetch(topaddr(regs.epc));
    let val = Store::from_val(inst).store_value(regs);

    match unsafe { VM.dev.write(fault_paddr, val) } {
        None => unsafe {
            VM.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(()) => {
            regs.epc = regs.epc + Instruction::len(inst);
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
            ));
        }
    }
}

pub fn do_guest_load_page_fault(sp: *mut usize /*regs: &mut Registers*/) {
    let regs = Registers::from(sp);
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(topaddr(regs.epc));

    match unsafe { VM.dev.read(fault_paddr) } {
        None => unsafe {
            VM.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    }
}

pub fn do_guest_instruction_page_fault(_sp: *mut usize /*_regs: &mut Registers*/) {
    unsafe {
        VM.map_guest_page(Hext::get_vs_fault_paddr() as usize);
    }
}

pub fn do_supervisor_external_interrupt(_sp: *mut usize /*_regs: &mut Registers*/) {
    // 物理PLICからペンディングビットを読み、クリアする
    let int_id = if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        i.get_pend_int()
    } else {
        0
    };

    // 仮想PLICへ書込み
    unsafe {
        match VM.dev.get_mut(0x0c20_1000) {
            // [todo fix] 割込み番号で検索できるようにする
            None => (),
            Some(d) => {
                d.interrupt(int_id as usize);
            }
        }
    }

    // 仮想外部割込みを発生させる
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
    ));

    // PLICでペンディングビットをクリア
    if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        i.set_comp_int(int_id);
    }
}

pub fn do_supervisor_timer_interrupt(_sp: *mut usize /*_regs: &mut Registers*/) {
    /* タイマの無効化 */
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    /* ゲストにタイマ割込みをあげる */
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
}

pub fn boot_linux() {
    
    unsafe {
        VM.setup();
    }

    /* 割込みを有効化 */
    Interrupt::enable_mask_s(
        Interrupt::bit(Interrupt::SUPERVISOR_TIMER_INTERRUPT)
            | Interrupt::bit(Interrupt::SUPERVISOR_EXTERNAL_INTERRUPT),
    );

    /* 割込みハンドラの登録 */
    cpu_mut().register_vector(
        TrapVector::SUPERVISOR_TIMER_INTERRUPT,
        do_supervisor_timer_interrupt,
    );
    cpu_mut().register_vector(
        TrapVector::SUPERVISOR_EXTERNAL_INTERRUPT,
        do_supervisor_external_interrupt,
    );

    /* 例外ハンドラの登録 */
    cpu_mut().register_vector(
        TrapVector::ENVIRONMENT_CALL_FROM_VSMODE,
        do_ecall_from_vsmode,
    );
    cpu_mut().register_vector(TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault);
    cpu_mut().register_vector(
        TrapVector::STORE_AMO_GUEST_PAGE_FAULT,
        do_guest_store_page_fault,
    );
    cpu_mut().register_vector(
        TrapVector::INSTRUCTION_GUEST_PAGE_FAULT,
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
        VM.cpu.register(0, boot_core); /* vcpu0 ... pcpu1 */
        match VM.cpu.get_mut(0) {
            None => (),
            Some(v) => {
                v.context.set(JUMP_ADDR, 0x8020_0000);
                v.context.set(ARG0, 0);
                v.context.set(ARG1, 0x8220_0000);
            }
        }
        /* RAM */
        VM.mem.register(0x8020_0000, 0x9020_0000, 0x1000_0000);
        VM.mem.register(0x8220_0000, 0x8220_0000, 0x2_0000); //FDTは物理メモリにマップ サイズは適当
        VM.mem.register(0x8810_0000, 0x88100000, 0x20_0000); //initrdも物理メモリにマップ サイズはrootfs.imgより概算

        /* MMIO */
        VM.dev.register(0x0c00_0000, 0x0400_0000, vplic);
    }
    /* コア1でLinuxを起動させる */
    create_task(2, boot_linux, boot_core);
}
