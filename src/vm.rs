use crate::constants::{Instruction, REGISTER_COUNTER, Register};

#[derive(Debug, Clone, Copy)]
pub struct VirtualMachine {
    memory: [u16; u16::MAX as usize],
    registers: [u16; REGISTER_COUNTER],
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            memory: [0; u16::MAX as usize],
            registers: [0; REGISTER_COUNTER],
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add {
                destination,
                source_1,
                source_2,
            } => self.add(destination, source_1, source_2),
            Instruction::AddImmediate {
                destination,
                source,
                value,
            } => (),
            _ => (),
        }
    }

    fn add(&mut self, destination: Register, source_1: Register, sorce_2: Register) {
        let value = self.registers[source_1 as usize] + self.registers[sorce_2 as usize];
        self.registers[destination as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_add() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 2;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 2);
    }
}
