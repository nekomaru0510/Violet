//! Exception

use crate::arch::rv64;
use rv64::csr::stval::*;

#[derive(Clone, Copy)]
pub struct Exception();

impl Exception {
    pub const INSTRUCTION_ADDRESS_MISALIGNED: usize = 0;
    pub const INSTRUCTION_ACCESS_FAULT: usize = 1;
    pub const ILLEGAL_INSTRUCTION: usize = 2;
    pub const BREAKPOINT: usize = 3;
    pub const LOAD_ADDRESS_MISALIGNED: usize = 4;
    pub const LOAD_ACCESS_FAULT: usize = 5;
    pub const STORE_AMO_ADDRESS_MISALIGNED: usize = 6;
    pub const STORE_AMO_ACCESS_FAULT: usize = 7;
    pub const ENVIRONMENT_CALL_FROM_UMODE_OR_VUMODE: usize = 8;
    pub const ENVIRONMENT_CALL_FROM_HSMODE: usize = 9;
    pub const ENVIRONMENT_CALL_FROM_VSMODE: usize = 10;
    pub const ENVIRONMENT_CALL_FROM_MMODE: usize = 11;
    pub const INSTRUCTION_PAGE_FAULT: usize = 12;
    pub const LOAD_PAGE_FAULT: usize = 13;
    pub const STORE_AMO_PAGE_FAULT: usize = 15;
    pub const INSTRUCTION_GUEST_PAGE_FAULT: usize = 20;
    pub const LOAD_GUEST_PAGE_FAULT: usize = 21;
    pub const VIRTUAL_INSTRUCTION: usize = 22;
    pub const STORE_AMO_GUEST_PAGE_FAULT: usize = 23;

    pub fn bit(val: usize) -> usize {
        1 << val
    }

    pub fn get_fault_address() -> u64 {
        Stval::get()
    }
}
