//! Hypervisor Extension

use core::intrinsics::transmute;

use crate::arch::rv64;
use crate::arch::traits::hypervisor::HypervisorT;
use crate::arch::traits::TraitArch;
use crate::arch::traits::mmu::{TraitPageTable};

use rv64::Rv64;
use rv64::regs::Registers;
use rv64::mmu::{sv39, sv48};
use rv64::mmu::get_new_page_table_addr;
use rv64::trap::exc::Exception;
use rv64::trap::int::Interrupt;
use rv64::trap::TrapVector;
use rv64::vscontext::VsContext;
use rv64::PagingMode;
use rv64::csr::hcounteren::*;
use rv64::csr::hedeleg::*;
use rv64::csr::hgatp;
use rv64::csr::hgatp::*;
use rv64::csr::hgeie::*;
use rv64::csr::hideleg::*;
use rv64::csr::hie::*;
use rv64::csr::htval::*;
use rv64::csr::hvip::*;
use rv64::csr::vsatp::*;
use rv64::csr::vsatp;
use rv64::csr::vstval::*;
use rv64::csr::vstvec::*;
use rv64::csr::vsie::*;
use rv64::csr::sstatus;
use rv64::csr::sstatus::*;
use rv64::csr::vsstatus;
use rv64::csr::vsstatus::*;
use rv64::csr::vscause::*;
use rv64::csr::scause::*;
use rv64::csr::stval::*;
use rv64::csr::sepc::*;
use rv64::csr::vsepc::*;

#[derive(Clone)]
pub struct Hext {}

impl HypervisorT for Hext {
    type Context = VsContext;
    
    fn init() {
        Hext::init();
    }
    
    // Reset virtual machine registers
    fn reset() {
        Hext::set_vs_pagetable(0);
    }

    fn hook(vecid: usize, func: fn(regs: *mut usize)) {
        if vecid > TrapVector::INTERRUPT_OFFSET {
            Self::clear_delegation_int(Interrupt::bit(vecid));
        } else {
            Self::clear_delegation_exc(Exception::bit(vecid));
        }
        let _ = Rv64::register_vector(vecid, func);
    }

    fn mmu_enable() {
        Self::set_table_addr(get_new_page_table_addr());
        Self::set_paging_mode(PagingMode::Sv48x4);
    }

    fn map_vaddr(paddr: usize, vaddr: usize, size: usize) {
        match Hext::get_paging_mode() {
            PagingMode::Sv39x4 => {
                unsafe {
                    transmute::<usize, &mut sv39::PageTableSv39>(
                        Hext::get_table_addr()
                    ).map_vaddr(paddr, vaddr);
                }
            }
            PagingMode::Sv48x4 => {
                unsafe {
                    transmute::<usize, &mut sv48::PageTableSv48>(
                        Hext::get_table_addr()
                    ).map_vaddr(paddr, vaddr);
                }
            }
            _ => {}
        }
    }

    // Translate virtual address to physical address
    fn v2p(vaddr: usize) -> usize {
        match Hext::get_paging_mode() {
            PagingMode::Sv39x4 => {
                unsafe {
                    transmute::<usize, &mut sv39::PageTableSv39>(
                        Hext::get_table_addr()
                    ).v2p(vaddr)
                }
            }
            PagingMode::Sv48x4 => {
                unsafe {
                    transmute::<usize, &mut sv48::PageTableSv48>(
                        Hext::get_table_addr()
                    ).v2p(vaddr)
                }
            }
            _ => 0
        }
    }

    // Redirect exceptions and interrupts to the guest OS when trapping
    fn redirect_to_guest(regs: &mut Registers) {
        // 1. vsstatus.SPP = sstatus.SPP
        match Sstatus::read(sstatus::SPP) {
            1 => Vsstatus::write(vsstatus::SPP, vsstatus::SPP::SET),
            0 => Vsstatus::write(vsstatus::SPP, vsstatus::SPP::CLEAR),
            _ => (),
        }
    
        // 2. vsstatus.SPIE = vsstatus.SIE
        let _s = Vsstatus::read(vsstatus::SIE);
        match Vsstatus::read(vsstatus::SIE) {
            1 => Vsstatus::write(vsstatus::SPIE, vsstatus::SPIE::SET),
            0 => Vsstatus::write(vsstatus::SPIE, vsstatus::SPIE::CLEAR),
            _ => (),
        }
        let _s2 = Vsstatus::read(vsstatus::SIE);
        let _v = Vsie::get();
        // vsstatus.SIE = 0
        Vsstatus::write(vsstatus::SIE, vsstatus::SIE::CLEAR);
    
        // vscause = scause
        Vscause::set(Scause::get());
        // vstval = stval
        Vstval::set(Stval::get());
        // vsepc = sepc
        Vsepc::set(Sepc::get());
    
        // 3. sepc = vstvec
        (*(regs)).epc = Vstvec::get() as usize;
    
        // 4. sstatus.SPP = 1
        Sstatus::write(sstatus::SPP, sstatus::SPP::SET);
    
        // 5. sret
    }
}

impl Hext {
    pub fn init() {
        Self::set_delegation_exc(
            Exception::bit(Exception::INSTRUCTION_ADDRESS_MISALIGNED)
                | Exception::bit(Exception::BREAKPOINT)
                | Exception::bit(Exception::ENVIRONMENT_CALL_FROM_UMODE_OR_VUMODE)
                | Exception::bit(Exception::INSTRUCTION_PAGE_FAULT)
                | Exception::bit(Exception::LOAD_PAGE_FAULT)
                | Exception::bit(Exception::STORE_AMO_PAGE_FAULT),
        );

        Self::set_delegation_int(
            Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_SOFTWARE_INTERRUPT)
                | Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT)
                | Interrupt::bit(Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT),
        );

        Self::flush_vsmode_interrupt(0xffff_ffff_ffff_ffff);
        Self::enable_vsmode_counter_access(0xffff_ffff);
    }

    // Enable specified interrupt in hypervisor mode
    pub fn enable_mask_h(int_mask: usize) {
        let hint_mask = 0x1444 & int_mask;
        Hie::set(Hie::get() | hint_mask as u64);
    }

    // Disable specified interrupt in hypervisor mode
    pub fn disable_mask_h(int_mask: usize) {
        let hint_mask = 0x1444 & int_mask;
        Hie::set(Hie::get() & !(hint_mask as u64));
    }

    /* VS-modeへの割込み移譲を設定 */
    pub fn set_delegation_int(int_mask: usize) {
        Hideleg::set(Hideleg::get() | int_mask as u64);
    }

    /* VS-modeへの割込み移譲を解除 */
    pub fn clear_delegation_int(int_mask: usize) {
        Hideleg::set(Hideleg::get() & !(int_mask as u64));
    }

    // Set exception delegation to VS-mode
    pub fn set_delegation_exc(exc_mask: usize) {
        Hedeleg::set(Hedeleg::get() | exc_mask as u64);
    }

    // Clear exception delegation to VS-mode
    pub fn clear_delegation_exc(exc_mask: usize) {
        Hedeleg::set(Hedeleg::get() & !(exc_mask as u64));
    }

    // Raise a virtual interrupt to VS-mode
    pub fn assert_vsmode_interrupt(int_mask: usize) {
        Hvip::set(int_mask as u64);
    }

    // Clear the interrupt of VS-mode
    pub fn flush_vsmode_interrupt(int_mask: usize) {
        let mask = !(int_mask) & Hvip::get() as usize;
        Hvip::set(mask as u64);
    }

    // Enable specified external interrupt
    pub fn enable_exint_mask_h(int_mask: usize) {
        Hgeie::set(Hgeie::get() | int_mask as u64);
    }

    // Disable specified external interrupt
    pub fn disable_exint_mask_h(int_mask: usize) {
        Hgeie::set(Hgeie::get() & !(int_mask as u64));
    }

    // Set the counteren register of VS-mode
    pub fn enable_vsmode_counter_access(counter_mask: usize) {
        Hcounteren::set(Hcounteren::get() | counter_mask as u32);
    }

    // Clear the counteren register of VS-mode
    pub fn disable_vsmode_counter_access(counter_mask: usize) {
        Hcounteren::set(Hcounteren::get() & !(counter_mask as u32));
    }

    // Set the mode of the page table provided by HS-mode
    pub fn set_paging_mode(mode: PagingMode) {
        match mode {
            PagingMode::Bare => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::BARE);
            }
            PagingMode::Sv39x4 => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::SV39X4);
            }
            PagingMode::Sv48x4 => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::SV48X4);
            }
            PagingMode::Sv57x4 => {
                Hgatp::write(hgatp::MODE, hgatp::MODE::SV57X4);
            }
        };
    }

    // Get the mode of the page table provided by HS-mode
    pub fn get_paging_mode() -> PagingMode {
        match Hgatp::read(hgatp::MODE) {
            hgatp::MODE::BARE => PagingMode::Bare,
            hgatp::MODE::SV39X4 => PagingMode::Sv39x4,
            hgatp::MODE::SV48X4 => PagingMode::Sv48x4,
            hgatp::MODE::SV57X4 => PagingMode::Sv57x4,
            _ => PagingMode::Bare,
        }
    }

    // Set the address of the page table provided by HS-mode
    pub fn set_table_addr(table_addr: usize) {
        Hgatp::write(hgatp::PPN, hgatp::PPN::CLEAR);
        let current = Hgatp::get();
        Hgatp::set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    // Get the address of the page table provided by HS-mode
    pub fn get_table_addr() -> usize {
        (Hgatp::read(hgatp::PPN) << 12) as usize
    }

    // Set the address of the page table set by the guest OS
    pub fn set_vs_pagetable(table_addr: usize) {
        Vsatp::write(vsatp::PPN, vsatp::PPN::CLEAR);
        let current = Vsatp::get();
        Vsatp::set(current | ((table_addr as u64 >> 12) & 0x3f_ffff));
    }

    // Get the address of the page table set by the guest OS
    pub fn get_vs_pagetable() -> u64 {
        (Vsatp::get() & 0x0fff_ffff_ffff) << 12
    }

    // Get the address at the time of page fault
    pub fn get_vs_fault_address() -> u64 {
        Vstval::get()
    }

    pub fn get_vs_fault_paddr() -> u64 {
        Htval::get() << 2
    }

    pub fn set_vs_vector(val: u64) {
        Vstvec::set(val);
    }

    pub fn get_vs_vector() -> u64 {
        Vstvec::get()
    }

}

