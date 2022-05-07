//! Sifive Platform-Level Interrupt Controller(PLIC)

use core::ptr::{write_volatile, read_volatile};

/* ドライバ用トレイト */
use crate::driver::traits::intc::TraitIntc;

#[derive(Clone)]
pub struct Plic {
    base: usize,
}

//const SOURCE_1_PRIO: usize = 0x4;
//const START_OF_PENDING_ARRAY: usize = 0x1000;
const START_HART0_INT_ENABLE: usize = 0x2000;
//const HART0_PRIO_THRESHOLD: usize = 0x2_0000;
const HART0_CLAIM_COMPLETE: usize = 0x20_1004;   
//const HART0_CLAIM_COMPLETE: usize = 0x20_0000;   

impl TraitIntc for Plic {

    /* 割込みの有効化 */
    fn enable_intrrupt(&self, id: u32) {
        self.set_enable(id);
    }
    
    /* 割込みの無効化 */
    fn disable_interrupt(&self, id: u32) {
        self.clear_enable(id);
    }

    /* 最高優先度のペンディング状態の割込み番号を取得 */
    fn get_pend_int(&self) -> u32 {
        self.get_claim_complete()        
    }

    /* 処理完了した割込み番号を格納 */
    fn set_comp_int(&self, id: u32) {
        self.set_claim_complete(id);
    }

}

impl Plic {

    pub fn new(base: usize) -> Self {
        Plic { base, }
    }

    pub fn set_enable(&self, id:u32) {
        let offset = ((id / 32)*4) as usize;
        let val = 0x01 << (id % 32) as u32;

        unsafe {
            write_volatile((self.base + START_HART0_INT_ENABLE + offset) as *mut u64, val);
        }
    }

    /* [todo fix] clear処理にする */
    pub fn clear_enable(&self, id: u32) {
        let offset = ((id / 32)*4) as usize;
        let val = 0x01 << (id % 32) as u32;

        unsafe {
            write_volatile((self.base + START_HART0_INT_ENABLE + offset) as *mut u64, val);
        }
    }

    pub fn get_claim_complete(&self) -> u32 {
        unsafe { read_volatile((self.base + HART0_CLAIM_COMPLETE) as *const u32) }
    }

    pub fn set_claim_complete(&self, id: u32) {
        unsafe {
            write_volatile((self.base + HART0_CLAIM_COMPLETE) as *mut u32, id);
        }
    }

}

