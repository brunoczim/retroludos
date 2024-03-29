#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntRegister {
    I0 = 0b_000,
    I1 = 0b_001,
    I2 = 0b_010,
    I3 = 0b_011,
    I4 = 0b_100,
    I5 = 0b_101,
    I6 = 0b_110,
    I7 = 0b_111,
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
    R0 = 0b_00,
    R1 = 0b_01,
    R2 = 0b_10,
    R3 = 0b_11,
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
    NoExplSource = 0b_011_1111,

    /// Bits except operands:   13  = 16 - 3
    /// Opcode bits:            6   = 13 - 7
    NoExplDest = 0b_011_1110,

    /// Bits except operands:   12  = 16 - (2 * 2 = 4)
    /// Opcode bits:            6   = 12 - 6
    RatioUnaryOp = 0b_01_1111,

    /// Bits except operands:   12  = 16 - (2 * 2 = 4)
    /// Opcode bits:            6   = 12 - 6
    RatioBinNoExplDest = 0b_01_1110,

    /// Bits except operands:   11  = 16 - (2 + 3 = 5)
    /// Opcode bits:            6   = 11 - 5
    IntToRatio = 0b_0_1111,

    /// Bits except operands:   11  = 16 - (2 + 3 = 5)
    /// Opcode bits:            6   = 11 - 5
    RatioToInt = 0b_0_1110,

    /// Bits except operands:   10  = 16 - (3 * 2 = 6)
    /// Opcode bits:            6   = 10 - 4
    IntUnaryOp = 0b_0111,

    /// Bits except operands:   10  = 16 - (3 * 2 = 6)
    /// Opcode bits:            6   = 10 - 4
    IntBinNoExplDest = 0b_0110,

    /// Bits except operands:   10  = 16 - (2 * 3 = 6)
    /// Opcode bits:            7   = 10 - 3
    RatioBinaryOp = 0b_011,

    /// Bits except operands:   8   = 16 - 8
    /// Opcode bits:            5   = 8 - 3
    NoExplDestImm = 0b_010,

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
    NoExplSource(NoExplSourceInstr),
    NoExplDest(NoExplDestInstr),
    IntUnaryOp(IntUnaryOpInstr),
    IntBinaryOp(IntBinaryOpInstr),
    IntBinNoExplDest(IntBinNoExplDestInstr),
    RatioUnaryOp(RatioUnaryOpInstr),
    RatioBinaryOp(RatioBinaryOpInstr),
    RatioBinNoExplDest(RatioBinNoExplDestInstr),
    IntToRatio(IntToRatioInstr),
    RatioToInt(RatioToIntInstr),
    Immediate(ImmediateInstr),
    NoExplDestImm(NoExplDestImmInstr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoExplOperandsInstr {
    pub opcode: NoExplOperandsOpcode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoExplOperandsOpcode {
    Nop = 0b_0000_0000,
    Ret = 0b_0000_0001,
    Halt = 0b_000_0010,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoExplSourceInstr {
    pub opcode: NoExplSourceOpcode,
    pub dest: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoExplSourceOpcode {
    Pop = 0b_00_0000,
    Ldsp = 0b_00_0001,
    Ldf = 0b_00_0010,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoExplDestInstr {
    pub opcode: NoExplDestOpcode,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoExplDestOpcode {
    Push = 0b_00_0000,
    Stsp = 0b_00_0001,
    Addsp = 0b_00_0010,
    Subsp = 0b_00_0011,
    Stf = 0b_00_0100,
    Jabs = 0b_00_0101,
    Cabs = 0b_00_0110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntUnaryOpInstr {
    pub opcode: IntUnaryOpOpcode,
    pub dest: IntRegister,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntUnaryOpOpcode {
    Cpy = 0b_00_0000,
    St = 0b_00_0001,
    Not = 0b_00_0010,
    Neg = 0b_00_0011,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntBinaryOpInstr {
    pub opcode: IntBinaryOpOpcode,
    pub dest: IntRegister,
    pub lhs: IntRegister,
    pub rhs: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntBinaryOpOpcode {
    Or = 0b_00_0000,
    And = 0b_00_0001,
    Xor = 0b_00_0010,
    Shl = 0b_00_0011,
    Shr = 0b_00_0100,
    Rol = 0b_00_0101,
    Ror = 0b_00_0110,
    Add = 0b_00_0111,
    Sub = 0b_00_1000,
    Mul = 0b_00_1001,
    Mulu = 0b_00_1010,
    Muls = 0b_00_1011,
    Quot = 0b_00_1100,
    Rem = 0b_00_1101,
    Div = 0b_00_1110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntBinNoExplDestInstr {
    pub opcode: IntBinNoExplDestOpcode,
    pub lhs: IntRegister,
    pub rhs: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntBinNoExplDestOpcode {
    Cmp = 0b_000_0000,
    Ld = 0b_000_0001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioUnaryOpInstr {
    pub opcode: RatioUnaryOpOpcode,
    pub dest: RatioRegister,
    pub source: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioUnaryOpOpcode {
    Neg = 0b_00_0000,
    Inv = 0b_00_0001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioBinaryOpInstr {
    pub opcode: RatioBinaryOpOpcode,
    pub dest: RatioRegister,
    pub lhs: RatioRegister,
    pub rhs: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioBinaryOpOpcode {
    Addru = 0b_00_0000,
    Addrs = 0b_00_0001,
    Subru = 0b_00_0010,
    Subrs = 0b_00_0011,
    Mulru = 0b_00_0100,
    Mulrs = 0b_00_0101,
    Divru = 0b_00_0110,
    Divrs = 0b_00_0111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioBinNoExplDestInstr {
    pub opcode: RatioBinNoExplDestOpcode,
    pub lhs: RatioRegister,
    pub rhs: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioBinNoExplDestOpcode {
    Cmpr = 0b_00_0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntToRatioInstr {
    pub opcode: IntToRatioOpcode,
    pub dest: RatioRegister,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntToRatioOpcode {
    Rat = 0b_00_0000,
    Ldden = 0b_00_0001,
    Ldnum = 0b_00_0010,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RatioToIntInstr {
    pub opcode: RatioToIntOpcode,
    pub dest: IntRegister,
    pub source: RatioRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RatioToIntOpcode {
    Rnd = 0b_00_0000,
    Trnc = 0b_00_0001,
    Flr = 0b_00_0010,
    Ceil = 0b_00_0011,
    Stden = 0b_00_0100,
    Stnum = 0b_00_0101,
}

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
    Shli = 0b_100,
    Shri = 0b_101,
    Addi = 0b_110,
    Muli = 0b_111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoExplDestImmInstr {
    pub opcode: NoExplDestImmOpcode,
    pub immediate: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoExplDestImmOpcode {
    Jmp = 0b_0000,
    Call = 0b_0001,
    Jc = 0b_0010,
    Jnc = 0b_0011,
    Jb = 0b_0100,
    Jnb = 0b_0101,
    Jo = 0b_0110,
    Jno = 0b_0111,
    Js = 0b_1000,
    Jns = 0b_1001,
    Jz = 0b_1010,
    Jnz = 0b_1011,
    Ja = 0b_1100,
    Jge = 0b_1101,
    Jl = 0b_1110,
    Jbe = 0b_1111,
}
