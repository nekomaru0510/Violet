//! Hypervisor Status Register (hstatus)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hstatus [
        VSBE       OFFSET(5)  NUMBITS(1) [],    // エンディアンの設定
        GVA        OFFSET(6)  NUMBITS(1) [],    //
        SPV        OFFSET(7)  NUMBITS(1) [],    // トラップ前の仮想化状態(V)を示す
        SPVP       OFFSET(8)  NUMBITS(1) [],    // トラップ前の特権状態を示す
        HU         OFFSET(9)  NUMBITS(1) [],    // 1の場合、HU-modeがHS限定の命令を利用可能になる
        VGEIN      OFFSET(12)  NUMBITS(6) [],
        VTVM       OFFSET(20)  NUMBITS(1) [],   // VS-modeのsfence.vma or satpアクセスで例外を発生させる
        VTW        OFFSET(21)  NUMBITS(1) [],   // VS-modeのwfiで例外を発生させる
        VTSR       OFFSET(22)  NUMBITS(1) [],   // VS-modeのsretで例外を発生させる
        VSXL       OFFSET(32)  NUMBITS(2) []    // VS-modeのビットサイズを指定
    ]
}

#[derive(Clone)]
pub struct Hstatus;

impl RegisterReadWrite<u64, hstatus::Register> for Hstatus {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x600" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x600, $0" :: "r"(value) :: "volatile");
        }
    }
}
