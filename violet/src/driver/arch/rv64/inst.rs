//! RISC-V用命令

use crate::driver::arch::rv64::regs::Registers;

#[derive(Clone)]
pub struct Rv64Inst {}

impl Rv64Inst {
    pub const fn new() -> Self {
        Rv64Inst {}
    }

    pub fn do_ecall(
        &self,
        ext: i32,
        fid: i32,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> (usize, usize) {
        unsafe {
            let mut val: usize = 0;
            let mut err: usize = 0;

            asm! ("
            .align 8
                    addi a0, $2, 0
                    addi a1, $3, 0
                    addi a2, $4, 0
                    addi a3, $5, 0
                    addi a4, $6, 0
                    addi a5, $7, 0
                    addi a6, $8, 0
                    addi a7, $9, 0
                    ecall
                    addi $0, a0, 0
                    addi $1, a1, 0
            "
            : "+r"(err), "+r"(val)
            : "r"(arg0), "r"(arg1), "r"(arg2), "r"(arg3), "r"(arg4), "r"(arg5), "r"(fid), "r"(ext)
            : "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"
            : );

            return (err, val);
        }
    }

    pub fn jump_by_sret(&self, next_addr: usize, arg1: usize, arg2: usize) {
        if next_addr == 0 {
            unsafe {
                asm! ("
                .align 8
                        la  a0, 1f
                        csrw sepc, a0
                        sret
                1:
                        nop
                "
                :
                :
                :
                : "volatile");
            }
        } else {
            unsafe {
                asm! ("
                .align 8
                        csrw sepc, $0
                        addi a0, $1, 0
                        addi a1, $2, 0
                        sret
                "
                :
                : "r"(next_addr), "r"(arg1), "r"(arg2) 
                :
                : "volatile");
            }
        }
    }

    pub fn wfi(&self) {
        unsafe {
            asm! ("
            .align 8
                    wfi
            "
            :
            :
            :
            : "volatile");
        }
    }
}

// Instruction Analyzer(proto)

pub enum Instruction {
    LB,
    LH,
    LW,
    LD,
    SB,
    SH,
    SW,
    SD,
    CSD,
    CSW,
    CSQ,
    CLD,
    CLW,
    CLQ,
    UNIMP,
}

enum Opcode {
    Load = 0b0000011,
    Store = 0b0100011,
    UNKNOWN,
}
impl Opcode {
    pub fn from_inst(inst: usize) -> Self {
        match inst & (0b1111111) {
            0b0000011 => Opcode::Load,
            0b0100011 => Opcode::Store,
            _ => Opcode::UNKNOWN,
        }
    }
}

enum LoadFunct3 {
    LB = 0b000,
    LH = 0b001,
    LW = 0b010,
    LD = 0b011,
    UNKNOWN,
}
impl LoadFunct3 {
    pub fn from_inst(inst: usize) -> Self {
        match inst & ((0b111 << 12) >> 12) {
            0b000 => LoadFunct3::LB,
            0b001 => LoadFunct3::LH,
            0b010 => LoadFunct3::LW,
            0b011 => LoadFunct3::LD,
            _ => LoadFunct3::UNKNOWN,
        }
    }
}

enum StoreFunct3 {
    SB = 0b000,
    SH = 0b001,
    SW = 0b010,
    SD = 0b011,
    UNKNOWN,
}
impl StoreFunct3 {
    pub fn from_inst(inst: usize) -> Self {
        match inst & ((0b111 << 12) >> 12) {
            0b000 => StoreFunct3::SB,
            0b001 => StoreFunct3::SH,
            0b010 => StoreFunct3::SW,
            0b011 => StoreFunct3::SD,
            _ => StoreFunct3::UNKNOWN,
        }
    }
}

enum CompressedInst10 {
    RVC0 = 0b00,
    RVC1 = 0b01,
    RVC2 = 0b10,
    UNKNOWN,
}
impl CompressedInst10 {
    pub fn from_inst(inst: usize) -> Self {
        match inst & (0b11 << 0) {
            0b00 => CompressedInst10::RVC0,
            0b01 => CompressedInst10::RVC1,
            0b10 => CompressedInst10::RVC2,
            _ => CompressedInst10::UNKNOWN,
        }
    }
}

enum CompressedInst1513 {
    LQ = 0b001,
    LW = 0b010,
    LD = 0b011,
    SQ = 0b101,
    SW = 0b110,
    SD = 0b111,
    UNKNOWN,
}
impl CompressedInst1513 {
    pub fn from_inst(inst: usize) -> Self {
        match (inst & (0b111 << 13)) >> 13 {
            0b001 => CompressedInst1513::LQ,
            0b010 => CompressedInst1513::LW,
            0b011 => CompressedInst1513::LD,
            0b101 => CompressedInst1513::SQ,
            0b110 => CompressedInst1513::SW,
            0b111 => CompressedInst1513::SD,
            _ => CompressedInst1513::UNKNOWN,
        }
    }
}

pub fn inst_size(inst: usize) -> usize {
    if is_compressed(inst) {
        2
    } else {
        4
    }
}

pub fn is_compressed(inst: usize) -> bool {
    if inst & (0b11 << 0) == 0b11 {
        false
    } else {
        true
    }
}

pub fn get_store_value(inst: usize, regs: &Registers) -> usize {
    match analyze_instruction(inst) {
        Instruction::CSD | Instruction::CSW | Instruction::CSQ => {
            let f: CSFormat = CSFormat { inst };
            f.get_store_value(regs)
        }
        Instruction::SB | Instruction::SH | Instruction::SW | Instruction::SD => {
            let f: SFormat = SFormat { inst };
            f.get_store_value(regs)
        }
        _ => 0,
    }
}

pub fn get_load_reg(inst: usize) -> usize {
    match analyze_instruction(inst) {
        Instruction::CLD | Instruction::CLW | Instruction::CLQ => {
            let f: CLFormat = CLFormat { inst };
            f.get_load_reg(inst)
        }
        Instruction::LB | Instruction::LH | Instruction::LW | Instruction::LD => {
            let f: LFormat = LFormat { inst };
            f.get_load_reg(inst)
        }
        _ => 0,
    }
}

/* [todo fix] storeのみ判定可能 */
pub fn analyze_instruction(inst: usize) -> Instruction {
    if is_compressed(inst) {
        match CompressedInst10::from_inst(inst) {
            CompressedInst10::RVC0 => match CompressedInst1513::from_inst(inst) {
                CompressedInst1513::LQ => Instruction::CLQ,
                CompressedInst1513::LW => Instruction::CLW,
                CompressedInst1513::LD => Instruction::CLD,
                CompressedInst1513::SQ => Instruction::CSQ,
                CompressedInst1513::SW => Instruction::CSW,
                CompressedInst1513::SD => Instruction::CSD,
                _ => Instruction::UNIMP,
            },
            _ => Instruction::UNIMP,
        }
    } else {
        // S形式 .. op[6:0]
        match Opcode::from_inst(inst) {
            Opcode::Store => match StoreFunct3::from_inst(inst) {
                StoreFunct3::SB => Instruction::SB,
                StoreFunct3::SH => Instruction::SH,
                StoreFunct3::SW => Instruction::SW,
                StoreFunct3::SD => Instruction::SD,
                _ => Instruction::UNIMP,
            },
            Opcode::Load => match LoadFunct3::from_inst(inst) {
                LoadFunct3::LB => Instruction::LB,
                LoadFunct3::LH => Instruction::LH,
                LoadFunct3::LW => Instruction::LW,
                LoadFunct3::LD => Instruction::LD,
                _ => Instruction::UNIMP,
            },
            _ => Instruction::UNIMP,
        }
    }
}

pub struct LFormat {
    pub inst: usize,
}

impl LFormat {
    pub fn get_load_reg(&self, inst: usize) -> usize {
        //rd [11:7]
        (self.inst & (0b11111 << 7)) >> 7
    }
}

pub struct CLFormat {
    pub inst: usize,
}

impl CLFormat {
    pub fn get_load_reg(&self, inst: usize) -> usize {
        //rd [4:2]
        ((self.inst & (0b111 << 2)) >> 2) + 8
    }
}

pub struct SFormat {
    pub inst: usize,
}

impl SFormat {
    pub fn get_store_value(&self, regs: &Registers) -> usize {
        //rs2 [24:20]
        let reg = (self.inst & (0b11111 << 20)) >> 20;
        match reg {
            0..=0b11111 => regs.reg[reg],
            _ => 0,
        }
    }
}

pub struct CSFormat {
    pub inst: usize,
}

impl CSFormat {
    pub fn get_store_value(&self, regs: &Registers) -> usize {
        //rs2 [4:2]
        let reg = (self.inst & (0b111 << 2)) >> 2;
        match reg {
            0..=0b111 => regs.reg[reg + 8],
            _ => 0,
        }
    }
}
