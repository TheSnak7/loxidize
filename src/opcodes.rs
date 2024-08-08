use std::fmt::{self};

use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Op {
    ConstantSmall,
    Ret,
}

impl Op {
    // Provides the count of u8 operands of a given instruction
    pub fn operand_count(self) -> usize {
        match self {
            Op::Ret => 0,
            Op::ConstantSmall => 1,
        }
    }
}

impl From<Op> for u8 {
    fn from(value: Op) -> Self {
        value as u8
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Op::Ret => write!(f, "OP_RETURN"),
            Op::ConstantSmall => write!(f, "OP_CONSTANT_SMALL"),
        }
    }
}
