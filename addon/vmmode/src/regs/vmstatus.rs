//! Virtual mstatus csr

extern crate violet;
use violet::{bit_extract, bit_fill, bitfield, bit_set, bit_clear};
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::csr::vsstatus::Vsstatus;

pub struct Vmstatus {
    val: u64,
}

impl Vmstatus {
    pub fn new() -> Self {
        Vmstatus { val: 0 }
    }
}

impl VirtualRegisterT for Vmstatus {
    //type Regsize = u64;
    
    fn write(&mut self, val: u64) {
        bitfield!(SIE:[1,1]);
        bitfield!(MIE:[3,3]);
        bitfield!(SPIE:[5,5]);
        bitfield!(MPIE:[7,7]);
        bitfield!(SPP:[8,8]);
        bitfield!(MPP:[12,11]);

        // MIE -> SIE
        let mie = bit_extract!(val, MIE);
        self.val = bit_set!(self.val, SIE, mie);
        self.val = bit_set!(self.val, MIE, 0);
    
        // MPIE -> SPIE
        let mpie = bit_extract!(val, MPIE);
        self.val = bit_set!(self.val, SPIE, mpie);
        self.val = bit_set!(self.val, MPIE, 0);

        // MPP -> SPP
        let mpp = bit_extract!(val, MPP);
        self.val = bit_set!(self.val, SPP, mpp);
        self.val = bit_set!(self.val, MPP, 0);

        Vsstatus::set(self.val);
    }

    fn read(&mut self) -> u64 {
        bitfield!(SIE:[1,1]);
        bitfield!(MIE:[3,3]);
        bitfield!(SPIE:[5,5]);
        bitfield!(MPIE:[7,7]);
        bitfield!(MPP:[12,11]);

        // vsstatus::SIE -> MIE
        self.val = Vsstatus::get();
        let sie = bit_extract!(self.val, SIE);
        self.val = bit_set!(self.val, MIE, sie);
        self.val = bit_set!(self.val, SIE, 0);      // Clear SIE

        // vsstatus::SPIE -> MPIE
        let spie = bit_extract!(self.val, SPIE);
        self.val = bit_set!(self.val, MPIE, spie);
        self.val = bit_set!(self.val, SPIE, 0);     // Clear SPIE

        // MPP is fixed to 0b11
        self.val = bit_set!(self.val, MPP, 0x3);

        self.val
    }
}

