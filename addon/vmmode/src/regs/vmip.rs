//! Virtual mip csr

extern crate violet;
use violet::{bit_extract, bit_fill, bitfield, bit_set, bit_clear};
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::csr::vsip::Vsip;

pub struct Vmip {
    val: u64,
}

impl Vmip {
    pub fn new() -> Self {
        Vmip { val: 0 }
    }
}

impl VirtualRegisterT for Vmip {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        bitfield!(SSIP:[1,1]);
        bitfield!(MSIP:[3,3]);
        bitfield!(STIP:[5,5]);
        bitfield!(MTIP:[7,7]);
        bitfield!(SEIP:[9,9]);
        bitfield!(MEIP:[11,11]);
        // MSIP -> SSIP
        let msip = bit_extract!(val, MSIP);
        self.val = bit_set!(self.val, SSIP, msip);
        // MTIP -> STIP
        let mtip = bit_extract!(val, MTIP);
        self.val = bit_set!(self.val, STIP, mtip);
        // MEIP -> SEIP
        let meip = bit_extract!(val, MEIP);
        self.val = bit_set!(self.val, SEIP, meip);

        Vsip::set(self.val);
    }

    fn read(&mut self) -> u64 {
        bitfield!(SSIP:[1,1]);
        bitfield!(MSIP:[3,3]);
        bitfield!(STIP:[5,5]);
        bitfield!(MTIP:[7,7]);
        bitfield!(SEIP:[9,9]);
        bitfield!(MEIP:[11,11]);

        self.val = Vsip::get();
        // vsip::SSIP -> MSIP
        let ssip = bit_extract!(self.val, SSIP);
        self.val = bit_set!(self.val, MSIP, ssip);

        // vsip::STIP -> MTIP
        let stip = bit_extract!(self.val, STIP);
        self.val = bit_set!(self.val, MTIP, stip);

        // vsip::SEIP -> MEIP
        let seip = bit_extract!(self.val, SEIP);
        self.val = bit_set!(self.val, MEIP, seip);
        
        self.val
    }
}

