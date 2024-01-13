#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntRegister {
    I0 = 0,
    I1 = 1,
    I2 = 2,
    I3 = 3,
    I4 = 4,
    I5 = 5,
    I6 = 6,
    I7 = 7,
}

impl IntRegister {
    const I0_CODE: u8 = Self::I0.code();
    const I1_CODE: u8 = Self::I1.code();
    const I2_CODE: u8 = Self::I2.code();
    const I3_CODE: u8 = Self::I3.code();
    const I4_CODE: u8 = Self::I4.code();
    const I5_CODE: u8 = Self::I5.code();
    const I6_CODE: u8 = Self::I6.code();
    const I7_CODE: u8 = Self::I7.code();

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub const fn from_code(code: u8) -> Option<Self> {
        let this = match code {
            Self::I0_CODE => Self::I0,
            Self::I1_CODE => Self::I1,
            Self::I2_CODE => Self::I2,
            Self::I3_CODE => Self::I3,
            Self::I4_CODE => Self::I4,
            Self::I5_CODE => Self::I5,
            Self::I6_CODE => Self::I6,
            Self::I7_CODE => Self::I7,
            _ => return None,
        };
        Some(this)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioRegister {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
}

impl RatioRegister {
    const R0_CODE: u8 = Self::R0.code();
    const R1_CODE: u8 = Self::R1.code();
    const R2_CODE: u8 = Self::R2.code();
    const R3_CODE: u8 = Self::R3.code();

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub const fn from_code(code: u8) -> Option<Self> {
        let this = match code {
            Self::R0_CODE => Self::R0,
            Self::R1_CODE => Self::R1,
            Self::R2_CODE => Self::R2,
            Self::R3_CODE => Self::R3,
            _ => return None,
        };
        Some(this)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionTag {
    /// Bits except operands:   16  = 16 - 0
    /// Opcode bits:            8   = 16 - 8
    NoExplOperands = 0b_0111_1110,

    /// Bits except operands:   13  = 16 - 3
    /// Opcode bits:            6   = 13 - 7
    ImplicitSource = 0b_011_1111,

    /// Bits except operands:   13  = 16 - 3
    /// Opcode bits:            6   = 13 - 7
    ImplicitDest = 0b_011_1110,

    /// Bits except operands:   12  = 16 - (2 * 2 = 4)
    /// Opcode bits:            6   = 12 - 6
    RatioUnaryOp = 0b_01_1111,

    /// Bits except operands:   12  = 16 - (2 * 2 = 4)
    /// Opcode bits:            6   = 12 - 6
    RatioBinImplicitDest = 0b_01_1110,

    /// Bits except operands:   11  = 16 - (2 + 3 = 5)
    /// Opcode bits:            6   = 11 - 5
    IntToRatio = 0b_0_1111,

    /// Bits except operands:   11  = 16 - (2 + 3 = 5)
    /// Opcode bits:            6   = 11 - 5
    RatioToInt = 0b_0_1110,

    /// Bits except operands:   10  = 16 - (3 * 2 = 6)
    /// Opcode bits:            6   = 10 - 4
    IntUnaryOp = 0b_0111,

    /// Bits except operands:   10  = 16 - (2 * 3 = 6)
    /// Opcode bits:            6   = 10 - 4
    RatioBinaryOp = 0b_0110,

    /// Bits except operands:   10  = 16 - (3 * 2 = 6)
    /// Opcode bits:            7   = 10 - 3
    IntBinImplicitDest = 0b_011,

    /// Bits except operands:   8   = 16 - 8
    /// Opcode bits:            5   = 8 - 3
    ImplicitDestImm = 0b_010,

    /// Bits except operands:   7   = 16 - (3 * 3 = 9)
    /// Opcode bits:            5   = 7 - 2
    IntBinaryOp = 0b_01,

    /// Bits except operands:   5   = 16 - (3 + 8 = 11)
    /// Opcode bits:            3   = 5 - 2
    Immediate = 0b_00,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instruction {
    NoExplOperands(NoExplOperandsInstr),
    ImplicitSource(ImplicitSourceInstr),
    ImplicitDest(ImplicitDestInstr),
    IntUnaryOp(IntUnaryOpInstr),
    IntBinaryOp(IntBinaryOpInstr),
    IntBinImplicitDest(IntBinImplicitDestInstr),
    RatioUnaryOp(RatioUnaryOpInstr),
    RatioBinaryOp(RatioBinaryOpInstr),
    RatioBinImplicitDest(RatioBinImplicitDestInstr),
    IntToRatio(IntToRatioInstr),
    RatioToInt(RatioToIntInstr),
    Immediate(ImmediateInstr),
    ImplicitDestImm(ImplicitDestImmInstr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoExplOperandsInstr {
    pub opcode: NoExplOperandsOpcode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoExplOperandsOpcode {
    Nop = 0b_0000_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImplicitSourceInstr {
    pub opcode: ImplicitSourceOpcode,
    pub dest: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitSourceOpcode {
    Pop = 0b_00_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImplicitDestInstr {
    pub opcode: ImplicitDestOpcode,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitDestOpcode {
    Push = 0b_00_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntUnaryOpInstr {
    pub opcode: IntUnaryOpOpcode,
    pub dest: IntRegister,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntUnaryOpOpcode {
    Not = 0b_00_0000,
    Neg = 0b_00_0001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntBinaryOpInstr {
    pub opcode: IntBinaryOpOpcode,
    pub dest: IntRegister,
    pub lhs: IntRegister,
    pub rhs: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntBinaryOpOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntBinImplicitDestInstr {
    pub opcode: IntBinImplicitDestOpcode,
    pub lhs: IntRegister,
    pub rhs: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntBinImplicitDestOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioUnaryOpInstr {
    pub opcode: RatioUnaryOpOpcode,
    pub dest: RatioRegister,
    pub source: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioUnaryOpOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioBinaryOpInstr {
    pub opcode: RatioBinaryOpOpcode,
    pub dest: RatioRegister,
    pub lhs: RatioRegister,
    pub rhs: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioBinaryOpOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioBinImplicitDestInstr {
    pub opcode: RatioBinImplicitDestOpcode,
    pub lhs: RatioRegister,
    pub rhs: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioBinImplicitDestOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntToRatioInstr {
    pub opcode: IntToRatioOpcode,
    pub dest: RatioRegister,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntToRatioOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioToIntInstr {
    pub opcode: RatioToIntOpcode,
    pub dest: IntRegister,
    pub source: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioToIntOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImmediateInstr {
    pub opcode: ImmediateOpcode,
    pub dest: IntRegister,
    pub immediate: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImmediateOpcode {
    Ldil = 0b_000,
    Ldih = 0b_001,
    Oril = 0b_010,
    Orih = 0b_011,
    Andil = 0b_100,
    Andih = 0b_101,
    Addi = 0b_110,
    Muli = 0b_111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImplicitDestImmInstr {
    pub opcode: ImplicitDestImmOpcode,
    pub immediate: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitDestImmOpcode {
    Jmp = 0b_0000,
    Jc = 0b_0001,
    Jnc = 0b_0010,
    Jb = 0b_0011,
    Jnb = 0b_0100,
    Js = 0b_0101,
    Jns = 0b_0110,
    Jz = 0b_0111,
    Jnz = 0b_1000,
    Ja = 0b_1001,
    Jge = 0b_1010,
    Jl = 0b_1011,
    Jbe = 0b_1100,
}
