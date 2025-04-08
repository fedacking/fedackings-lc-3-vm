use crate::instructions::{Instruction, REGISTER_COUNTER, Register};

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
                mode,
                value,
            } => {
                if mode == 0 {
                    self.add(destination, source_1, source_2)
                } else {
                    self.add_immediate(destination, source_1, value);
                }
            }
            _ => (),
        }
    }

    fn add(&mut self, destination: Register, source_1: Register, sorce_2: Register) {
        let value =
            self.registers[source_1 as usize].wrapping_add(self.registers[sorce_2 as usize]);
        self.registers[destination as usize] = value;
    }

    fn add_immediate(&mut self, destination: Register, source_1: Register, mut value: u16) {
        value = value.wrapping_add(self.registers[source_1 as usize]);
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
            mode: 0,
            value: Register::R1 as u16,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 2;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 2);
    }

    #[test]
    fn vm_add_immediate() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 1,
            value: 5,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 2;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 5);
    }

    #[test]
    fn vm_add_immediate_negative() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 1,
            value: 0xFFFF, // -1
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 2;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 0xFFFF);
    }

    #[test]
    fn vm_add_immediate_negative_2() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 1,
            value: 0xFFF0, // -16
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 2;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize].wrapping_add(16), 0);
    }

    #[test]
    fn vm_add_shouldnt_panic_overflow() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 0,
            value: Register::R1 as u16, // 15
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R0 as usize] = u16::MAX;
        vm.registers[Register::R1 as usize] = 2;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 1);
    }

    #[test]
    fn vm_add_immediate_shouldnt_panic_overflow() {
        let instruction = Instruction::Add {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 1,
            value: 0x000F, // 15
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R0 as usize] = u16::MAX - 14;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 0);
    }
}
