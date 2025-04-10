#[derive(Debug)]
pub enum VMError {
    TrapCodeDecode { repr: u16 },
    RegisterDecode { repr: u16 },
    IO { err: std::io::Error },
    ReservedInstruction { repr: u16 }
}

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
pub enum TrapCode {
    Getc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    Out = 0x21,   /* output a character */
    Puts = 0x22,  /* output a word string */
    In = 0x23,    /* get character from keyboard, echoed onto the terminal */
    Putsp = 0x24, /* output a byte string */
    Halt = 0x25,  /* halt the program */
}

impl TrapCode {
    fn from_bits(value: u16) -> Result<Self, VMError> {
        match from_bits(value, 8, 0) {
            0x20 => Ok(TrapCode::Getc),
            0x21 => Ok(TrapCode::Out),
            0x22 => Ok(TrapCode::Puts),
            0x23 => Ok(TrapCode::In),
            0x24 => Ok(TrapCode::Putsp),
            0x25 => Ok(TrapCode::Halt),
            repr => {
                Err(VMError::TrapCodeDecode { repr })
            },
        }
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
    fn from_u16(value: u16) -> Result<Self, VMError> {
        match value {
            0 => Ok(Self::R0),
            1 => Ok(Self::R1),
            2 => Ok(Self::R2),
            3 => Ok(Self::R3),
            4 => Ok(Self::R4),
            5 => Ok(Self::R5),
            6 => Ok(Self::R6),
            7 => Ok(Self::R7),
            8 => Ok(Self::PC),
            9 => Ok(Self::Cond),
            _ => {
                /* consider blowing up */
                Err(VMError::RegisterDecode { repr: value })
            }
        }
    }

    fn from_bits(value: u16, offset: u16) -> Result<Self, VMError> {
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
    Branch {
        flag: u16,
        offset: u16,
    },
    Jump {
        source: Register,
    },
    JumpRegister {
        source: Register,
        mode: u16,
        offset: u16,
    },
    Store {
        source: Register,
        offset: u16,
    },
    StoreIndirect {
        source: Register,
        offset: u16,
    },
    StoreRegister {
        source_1: Register,
        source_2: Register,
        offset: u16,
    },
    Trap {
        routine: TrapCode,
    },
    Noop,
}

impl Instruction {
    pub fn encode(&self) -> u16 {
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
                ((OperationCode::LoadRegister as u16) << 12)
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
            Instruction::Branch { flag, offset } => {
                ((OperationCode::Branch as u16) << 12) + ((flag & 0x7) << 9) + (offset & 0x1FF)
            }
            Instruction::Jump { source } => {
                ((OperationCode::Jump as u16) << 12) + ((source as u16) << 6)
            }
            Instruction::JumpRegister {
                source,
                mode,
                offset,
            } => ((OperationCode::Jump as u16) << 12) + (mode << 11) + (offset & 0x7FF),
            Instruction::Store { source, offset } => {
                ((OperationCode::Store as u16) << 12) + ((source as u16) << 9) + (offset & 0x1FF)
            }
            Instruction::StoreIndirect { source, offset } => {
                ((OperationCode::StoreIndirect as u16) << 12)
                    + ((source as u16) << 9)
                    + (offset & 0x1FF)
            }
            Instruction::StoreRegister {
                source_1,
                source_2,
                offset,
            } => {
                ((OperationCode::StoreRegister as u16) << 12)
                    + ((source_1 as u16) << 9)
                    + ((source_2 as u16) << 6)
                    + (offset & 0x3F)
            }
            Instruction::Trap { routine } => {
                ((OperationCode::ExecuteTrap as u16) << 12) + (routine as u16)
            }
            Instruction::Noop => 0,
        }
    }

    pub fn decode(repr: u16) -> Result<Self, VMError> {
        let code = OperationCode::from_u16(repr >> 12);
        match code {
            OperationCode::Branch => Ok(Instruction::Branch {
                flag: from_bits(repr, 3, 9),
                offset: from_bits_signed(repr, 9, 0),
            }),
            OperationCode::Add => Ok(Instruction::Add {
                destination: Register::from_bits(repr, 9)?,
                source_1: Register::from_bits(repr, 6)?,
                source_2: Register::from_bits(repr, 0)?,
                mode: from_bits(repr, 1, 5),
                value: from_bits_signed(repr, 5, 0),
            }),
            OperationCode::Load => Ok(Instruction::Load {
                destination: Register::from_bits(repr, 9)?,
                offset: from_bits_signed(repr, 9, 0),
            }),
            OperationCode::Store => Ok(Instruction::Store {
                source: Register::from_bits(repr, 9)?,
                offset: from_bits_signed(repr, 9, 0),
            }),
            OperationCode::JumpRegister => Ok(Instruction::JumpRegister {
                source: Register::from_bits(repr, 6)?,
                mode: from_bits(repr, 1, 11),
                offset: from_bits_signed(repr, 11, 0),
            }),
            OperationCode::And => Ok(Instruction::And {
                destination: Register::from_bits(repr, 9)?,
                source_1: Register::from_bits(repr, 6)?,
                source_2: Register::from_bits(repr, 0)?,
                mode: from_bits(repr, 1, 5),
                value: from_bits_signed(repr, 5, 0),
            }),
            OperationCode::LoadRegister => Ok(Instruction::LoadRegister {
                destination: Register::from_bits(repr, 9)?,
                source: Register::from_bits(repr, 6)?,
                offset: from_bits_signed(repr, 6, 0),
            }),
            OperationCode::StoreRegister => Ok(Instruction::StoreRegister {
                source_1: Register::from_bits(repr, 9)?,
                source_2: Register::from_bits(repr, 6)?,
                offset: from_bits_signed(repr, 6, 0),
            }),
            OperationCode::Rti => Err(VMError::ReservedInstruction { repr }),
            OperationCode::Not => Ok(Instruction::Not {
                destination: Register::from_bits(repr, 9)?,
                source: Register::from_bits(repr, 6)?,
            }),
            OperationCode::LoadIndirect => Ok(Instruction::LoadIndirect {
                destination: Register::from_bits(repr, 9)?,
                offset: from_bits_signed(repr, 9, 0),
            }),
            OperationCode::StoreIndirect => Ok(Instruction::StoreIndirect {
                source: Register::from_bits(repr, 9)?,
                offset: from_bits_signed(repr, 9, 0),
            }),
            OperationCode::Jump => Ok(Instruction::Jump {
                source: Register::from_bits(repr, 6)?,
            }),
            OperationCode::Reserved => Err(VMError::ReservedInstruction { repr }),
            OperationCode::LoadEffectiveAddress => Ok(Instruction::LoadEffectiveAddress {
                destination: Register::from_bits(repr, 9)?,
                offset: from_bits_signed(repr, 9, 0),
            }),
            OperationCode::ExecuteTrap => Ok(Instruction::Trap {
                routine: TrapCode::from_bits(repr)?,
            }),
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
        assert_eq!(instruction, decoded.unwrap());
    }
}
