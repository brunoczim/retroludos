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
pub enum Instruction {
    NoExplOperands(NoExplOperandsInstr),
    ImplicitSource(ImplicitSourceInstr),
    ImplicitDest(ImplicitDestInstr),
    IntUnaryOp(IntUnaryOpInstr),
    IntBinaryOp(IntBinaryOpInstr),
    RatioUnaryOp(RatioUnaryOpInstr),
    RatioBinaryOp(RatioBinaryOpInstr),
    IntToRatio(IntToRatioInstr),
    RatioToInt(RatioToIntInstr),
    Immediate(ImmediateInstr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoExplOperandsInstr {
    pub opcode: NoExplOperandsOpcode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoExplOperandsOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImplicitSourceInstr {
    pub opcode: ImplicitSourceOpcode,
    pub dest: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitSourceOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImplicitDestInstr {
    pub opcode: ImplicitDestOpcode,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitDestOpcode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntUnaryOpInstr {
    pub opcode: IntUnaryOpOpcode,
    pub dest: IntRegister,
    pub source: IntRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntUnaryOpOpcode {}

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
pub enum ImmediateOpcode {}
