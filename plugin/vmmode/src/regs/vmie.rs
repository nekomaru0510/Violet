//! Virtual mie csr

extern crate violet;
use violet::{bit_extract, bit_fill, bitfield, bit_set, bit_clear};
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::csr::vsie::Vsie;
extern crate register;
use register::cpu::RegisterReadWrite;

pub struct Vmie {
    val: u64,
}

impl Vmie {
    pub fn new() -> Self {
        Vmie { val: 0 }
    }
}

impl VirtualRegisterT for Vmie {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        bitfield!(SSIE:[1,1]);
        bitfield!(MSIE:[3,3]);
        bitfield!(STIE:[5,5]);
        bitfield!(MTIE:[7,7]);
        bitfield!(SEIE:[9,9]);
        bitfield!(MEIE:[11,11]);
        /* MSIEの値をSSIEに設定 */
        let msie = bit_extract!(val, MSIE);
        self.val = bit_set!(self.val, SSIE, msie);
        /* MTIEの値をSTIEに設定 */
        let mtie = bit_extract!(val, MTIE);
        self.val = bit_set!(self.val, STIE, mtie);
        /* MEIEの値をSEIEに設定 */
        let meie = bit_extract!(val, MEIE);
        self.val = bit_set!(self.val, SEIE, meie);

        Vsie.set(self.val);
    }

    fn read(&mut self) -> u64 {
        bitfield!(SSIE:[1,1]);
        bitfield!(MSIE:[3,3]);
        bitfield!(STIE:[5,5]);
        bitfield!(MTIE:[7,7]);
        bitfield!(SEIE:[9,9]);
        bitfield!(MEIE:[11,11]);
        
        self.val = Vsie.get();
        /* vsieのSSIEをMSIEに設定 */
        let ssie = bit_extract!(self.val, SSIE);
        self.val = bit_set!(self.val, MSIE, ssie);

        /* vsieのSTIEをMTIEに設定 */
        let stie = bit_extract!(self.val, STIE);
        self.val = bit_set!(self.val, MTIE, stie);

        /* vsieのSEIEをMEIEに設定 */
        let seie = bit_extract!(self.val, SEIE);
        self.val = bit_set!(self.val, MEIE, seie);
        
        self.val
    }
}

