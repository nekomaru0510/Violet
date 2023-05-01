//! 仮想CPU

//use crate::driver::traits::cpu::context::TraitContext;
use crate::driver::traits::cpu::registers::TraitRegisters;

pub enum VcpuStatus {
    STOPPED,
    RUNNING,
    SUSPENDED,
}

pub struct VirtualCpu<T: TraitRegisters /*TraitContext*/> {
    vcpuid: usize, /* 仮想CPU番号 */
    cpuid: usize,  /* 対応する物理CPU番号 */
    //pub context: T,
    pub regs: T,
    status: VcpuStatus,
}

impl<T: TraitRegisters> VirtualCpu<T> {
    pub fn new(vcpuid: usize, cpuid: usize, regs: T) -> Self {
        VirtualCpu {
            vcpuid,
            cpuid,
            //context,
            regs,
            status: VcpuStatus::STOPPED,
        }
    }

    pub fn enter(&self, sp: &T) {
        // レジスタの復帰
        self.regs.restore_to(sp);
        //
    }
}
