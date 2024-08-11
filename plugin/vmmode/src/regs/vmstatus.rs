//! Virtual mstatus csr

extern crate violet;
use violet::{bit_extract, bit_fill, bitfield, bit_set, bit_clear};
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::csr::vsstatus::Vsstatus;
extern crate register;
use register::cpu::RegisterReadWrite;

pub struct Vmstatus {
    val: u64,
}

/*
bitfield!(SPIE:[5,5]);
bitfield!(UBE:[6,6]);
bitfield!(MPIE:[7,7]);
bitfield!(SPP:[8,8]);
*/
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

        /* MIEの値をSIEに設定 */
        let mie = bit_extract!(val, MIE);
        self.val = bit_set!(self.val, SIE, mie);
        self.val = bit_set!(self.val, MIE, 0);
    
        /* MPIEの値をSPIEに設定 */
        let mpie = bit_extract!(val, MPIE);
        self.val = bit_set!(self.val, SPIE, mpie);
        self.val = bit_set!(self.val, MPIE, 0);

        /* MPPの値をSPPに変換し、設定 */
        let mpp = bit_extract!(val, MPP);
        self.val = bit_set!(self.val, SPP, mpp);
        self.val = bit_set!(self.val, MPP, 0);

        Vsstatus.set(self.val);
    }

    fn read(&mut self) -> u64 {
        bitfield!(SIE:[1,1]);
        bitfield!(MIE:[3,3]);
        bitfield!(SPIE:[5,5]);
        bitfield!(MPIE:[7,7]);
        bitfield!(MPP:[12,11]);

        /* vsstatusのSIEをMIEに設定 */
        self.val = Vsstatus.get();
        let sie = bit_extract!(self.val, SIE);//vsstatus.read(vsstatus::SIE);
        self.val = bit_set!(self.val, MIE, sie);
        self.val = bit_set!(self.val, SIE, 0); /* SIEをクリア */

        /* vsstatus.SPIEをMPIEに設定 */
        let spie = bit_extract!(self.val, SPIE);//vsstatus.read(vsstatus::SIE);
        self.val = bit_set!(self.val, MPIE, spie);
        self.val = bit_set!(self.val, SPIE, 0); /* SPIEをクリア */

        /* MPPは0b11固定 */
        self.val = bit_set!(self.val, MPP, 0x3);

        self.val
    }
}

