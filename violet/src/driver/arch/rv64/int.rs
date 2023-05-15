//! RV64用 CPU内割込み機能モジュール

use super::csr::scause::*;
use super::csr::sie::*;
use super::csr::sip::*;
use super::csr::sstatus::*;

extern crate register;
use register::cpu::RegisterReadWrite;

#[derive(Clone)]
pub struct Rv64Int {
    pub sstatus: Sstatus,
    pub sip: Sip,
    pub sie: Sie,
}

impl Rv64Int {
    pub const fn new() -> Self {
        Rv64Int {
            sstatus: Sstatus {},
            sip: Sip {},
            sie: Sie {},
        }
    }

    /* supervisorモードの割込みを有効化 */
    pub fn enable_s(&self) {
        //self.sstatus.modify(sstatus::SIE::SET);
        Sstatus.modify(sstatus::SIE::SET);
    }

    /* supervisorモードの割込みを無効化 */
    pub fn disable_s(&self) {
        //self.sstatus.modify(sstatus::SIE::CLEAR);
        Sstatus.modify(sstatus::SIE::CLEAR);
    }

    /* supervisorモードの指定割込みを有効化 */
    pub fn enable_mask_s(&self, int_mask: usize) {
        let sint_mask = 0x222 & int_mask; // sieの有効ビットでマスク
        self.sie.set(self.sie.get() | sint_mask as u64);
    }

    /* supervisorモードの指定割込みを無効化 */
    pub fn disable_mask_s(&self, int_mask: usize) {
        let sint_mask = 0x222 & int_mask; // sieの有効ビットでマスク
                                          //self.sie.set(self.sie.get() & !(sint_mask as u64));
        Sie.set(self.sie.get() & !(sint_mask as u64));
    }
}

/*
 * scause等のビットで割り込みを判断する。
 * そのため、アーキテクチャ的には、63(64-1)個まで管理可能
 */
const MAX_NUM_OF_INTERRUPTS: usize = 63;

pub struct InterruptTable {
    interrupt: [Option<Interrupt>; MAX_NUM_OF_INTERRUPTS],
}

#[derive(Clone, Copy)]
pub struct Interrupt {
    int_id: usize,
    event_id: usize,
}

impl InterruptTable {
    pub fn new() -> Self {
        InterruptTable {
            interrupt: [None; MAX_NUM_OF_INTERRUPTS],
        }
    }

    pub fn register(&mut self, int: Interrupt) -> Result<(), ()> {
        if int.int_id >= MAX_NUM_OF_INTERRUPTS {
            Err(())
        } else {
            match &self.interrupt[int.int_id] {
                None => {
                    self.interrupt[int.int_id] = Some(int);
                    Ok(())
                }
                Some(i) => Err(()),
            }
        }
    }

    pub fn get_event(&self, int_id: usize) -> usize {
        if int_id >= MAX_NUM_OF_INTERRUPTS {
            0
        } else {
            match &self.interrupt[int_id] {
                None => 0,
                Some(i) => i.event_id,
            }
        }
    }

    pub fn current_event(&self) -> usize {
        let e: usize = Scause.read(scause::EXCEPTION) as usize;
        self.get_event(e)
    }
}

/* 割込み */

#[derive(Clone, Copy)]
pub enum DefaultInterrupt {
    SupervisorSoftwareInterrupt = 1, //Interrupt{int_id: 1, event_id: 1},
    VirtualSupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    SupervisorTimerInterrupt = 5,
    VirtualSupervisorTimerInterrupt,
    MachineTimerInterrupt,
    SupervisorExternalInterrupt = 9,
    VirtualSupervisorExternalInterrupt,
    MachineExternalInterrupt,
    SupervisorGuestExternalInterrupt = 12,
    //CustomInterrupt(usize),
}
/*
impl Interrupt {
    pub fn mask(&self) -> usize {
        1 << *self as usize
    }
}
*/

#[test_case]
fn test_interrupt() -> Result<(), &'static str> {
    let mut table = InterruptTable::new();
    /* デフォルト割込み登録 */
    //table.register(DefaultInterrupt::SupervisorSoftwareInterrupt);
    /* 新規割込み登録 */
    let int = Interrupt {
        int_id: 33,
        event_id: 8,
    };
    table.register(int);

    // イベントID取得
    table.current_event();

    Ok(())
}
