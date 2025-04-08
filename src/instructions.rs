/// Utility function that allows us to quickly just grab the specified bits
fn from_bits(value: u16, size: u16, offset: u16) -> u16 {
    (value >> offset) & ((1 << size) - 1)
}

/// Utility function that grabs the u16 and extends
/// the sign by adding 1s to the missing bits. This conforms to the two's
/// compliment signed number scheme
fn from_bits_signed(value: u16, size: u16, offset: u16) -> u16 {
    if ((value >> (size - 1)) & 1) == 1 {
        let mask = 0xFFFF << (size - 1);
        from_bits(value, size - 1, offset) | mask
    } else {
        from_bits(value, size - 1, offset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OperationCode {
    Branch,
    Add,
    Load,
    Store,
    JumpRegister,
    And,
    LoadRegister,
    StoreRegister,
    /// Unused
    Rti,
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    /// Unused
    Reserved,
    LoadEffectiveAddress,
    ExecuteTrap,
}

impl OperationCode {
    fn from_u16(value: u16) -> Self {
        match value {
            0 => Self::Branch,
            1 => Self::Add,
            2 => Self::Load,
            3 => Self::Store,
            4 => Self::JumpRegister,
            5 => Self::And,
            6 => Self::LoadRegister,
            7 => Self::StoreRegister,
            8 => Self::Rti,
            9 => Self::Not,
            10 => Self::LoadIndirect,
            11 => Self::StoreIndirect,
            12 => Self::Jump,
            14 => Self::LoadEffectiveAddress,
            15 => Self::ExecuteTrap,
            _ => Self::Reserved, // We create noops for any other operation code
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionFlag {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC, /* program counter */
    Cond,
}
/// Number of available registers
pub const REGISTER_COUNTER: usize = 10;

impl Register {
    fn from_u16(value: u16) -> Self {
        match value {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::R6,
            7 => Self::R7,
            8 => Self::PC,
            9 => Self::Cond,
            _ => {
                /* consider blowing up */
                todo!()
            }
        }
    }

    fn from_bits(value: u16, offset: u16) -> Self {
        Self::from_u16(from_bits(value, 3, offset))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Add {
        destination: Register,
        source_1: Register,
        source_2: Register,
        mode: u16,
        value: u16,
    },
    And {
        destination: Register,
        source_1: Register,
        source_2: Register,
        mode: u16,
        value: u16,
    },
    Not {
        destination: Register,
        source: Register,
    },
    Load {
        destination: Register,
        offset: u16,
    },
    LoadRegister {
        destination: Register,
        source: Register,
        offset: u16,
    },
    LoadIndirect {
        destination: Register,
        offset: u16,
    },
    LoadEffectiveAddress {
        destination: Register,
        offset: u16,
    },
    Noop,
}

impl Instruction {
    fn encode(&self) -> u16 {
        match *self {
            Instruction::Add {
                destination,
                source_1,
                source_2: _,
                mode,
                value,
            } => {
                ((OperationCode::Add as u16) << 12)
                    + ((destination as u16) << 9)
                    + ((source_1 as u16) << 6)
                    + (mode << 5)
                    + (value & 0x1F) // Kinda hack because value contains all of the bits of source_2
            }
            Instruction::And {
                destination,
                source_1,
                source_2,
                mode,
                value,
            } => {
                ((OperationCode::And as u16) << 12)
                    + ((destination as u16) << 9)
                    + ((source_1 as u16) << 6)
                    + (mode << 5)
                    + (value & 0x1F) // Kinda hack because value contains all of the bits of source_2
            }
            Instruction::Not {
                destination,
                source,
            } => {
                ((OperationCode::Not as u16) << 12)
                    + ((destination as u16) << 9)
                    + ((source as u16) << 6)
            }
            Instruction::Load {
                destination,
                offset,
            } => {
                ((OperationCode::Load as u16) << 12)
                    + ((destination as u16) << 9)
                    + (offset & 0x1FF)
            }
            Instruction::LoadRegister {
                destination,
                source,
                offset,
            } => {
                ((OperationCode::Load as u16) << 12)
                    + ((destination as u16) << 9)
                    + ((source as u16) << 6)
                    + (offset & 0x3F)
            }
            Instruction::LoadIndirect {
                destination,
                offset,
            } => {
                ((OperationCode::LoadIndirect as u16) << 12)
                    + ((destination as u16) << 9)
                    + (offset & 0x1FF)
            }
            Instruction::LoadEffectiveAddress {
                destination,
                offset,
            } => {
                ((OperationCode::LoadEffectiveAddress as u16) << 12)
                    + ((destination as u16) << 9)
                    + (offset & 0x1FF)
            }
            Instruction::Noop => 0,
        }
    }

    fn decode(repr: u16) -> Self {
        let code = OperationCode::from_u16(repr >> 12);
        match code {
            OperationCode::Branch => todo!(),
            OperationCode::Add => Instruction::Add {
                destination: Register::from_bits(repr, 9),
                source_1: Register::from_bits(repr, 6),
                source_2: Register::from_bits(repr, 0),
                mode: from_bits(repr, 1, 5),
                value: from_bits_signed(repr, 5, 0),
            },
            OperationCode::Load => Instruction::Load {
                destination: Register::from_bits(repr, 9),
                offset: from_bits_signed(repr, 9, 0),
            },
            OperationCode::Store => todo!(),
            OperationCode::JumpRegister => todo!(),
            OperationCode::And => Instruction::And {
                destination: Register::from_bits(repr, 9),
                source_1: Register::from_bits(repr, 6),
                source_2: Register::from_bits(repr, 0),
                mode: from_bits(repr, 1, 5),
                value: from_bits_signed(repr, 5, 0),
            },
            OperationCode::LoadRegister => Instruction::LoadRegister {
                destination: Register::from_bits(repr, 9),
                source: Register::from_bits(repr, 6),
                offset: from_bits_signed(repr, 6, 0),
            },
            OperationCode::StoreRegister => todo!(),
            OperationCode::Rti => todo!(),
            OperationCode::Not => Instruction::Not {
                destination: Register::from_bits(repr, 9),
                source: Register::from_bits(repr, 6),
            },
            OperationCode::LoadIndirect => Instruction::LoadIndirect {
                destination: Register::from_bits(repr, 9),
                offset: from_bits_signed(repr, 9, 0),
            },
            OperationCode::StoreIndirect => todo!(),
            OperationCode::Jump => todo!(),
            OperationCode::Reserved => todo!(),
            OperationCode::LoadEffectiveAddress => Instruction::LoadEffectiveAddress {
                destination: Register::from_bits(repr, 9),
                offset: from_bits_signed(repr, 9, 0),
            },
            OperationCode::ExecuteTrap => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_add() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 0,
            value: Register::R1 as u16,
        };
        let encoded = instruction.encode();
        assert_eq!(0x1001, encoded);
    }

    #[test]
    fn decode_add() {
        let decoded = Instruction::decode(0x1001);
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 0,
            value: Register::R1 as u16,
        };
        assert_eq!(instruction, decoded);
    }
}
