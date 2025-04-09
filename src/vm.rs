use crate::instructions::{ConditionFlag, Instruction, REGISTER_COUNTER, Register};

#[derive(Debug, Clone, Copy)]
pub struct VirtualMachine {
    memory: [u16; u16::MAX as usize],
    registers: [u16; REGISTER_COUNTER],
    running: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            memory: [0; u16::MAX as usize],
            registers: [0; REGISTER_COUNTER],
            running: false,
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
                    self.add(destination, source_1, source_2);
                } else {
                    self.add_immediate(destination, source_1, value);
                }
            }
            Instruction::And {
                destination,
                source_1,
                source_2,
                mode,
                value,
            } => {
                if mode == 0 {
                    self.and(destination, source_1, source_2);
                } else {
                    self.and_immediate(destination, source_1, value);
                }
            }
            Instruction::Not {
                destination,
                source,
            } => self.not(destination, source),
            Instruction::Load {
                destination,
                offset,
            } => self.load(destination, offset),
            Instruction::LoadRegister {
                destination,
                source,
                offset,
            } => self.load_register(destination, source, offset),
            Instruction::LoadIndirect {
                destination,
                offset,
            } => self.load_indirect(destination, offset),
            Instruction::LoadEffectiveAddress {
                destination,
                offset,
            } => self.load_effective_address(destination, offset),
            Instruction::Branch { flag, offset } => self.branch(flag, offset),
            Instruction::Jump { source } => self.jump(source),
            Instruction::JumpRegister {
                source,
                mode,
                offset,
            } => {
                if mode == 1 {
                    self.jump_immediate(offset);
                } else {
                    self.jump_register(source);
                }
            }
            Instruction::Store { source, offset } => self.store(source, offset),
            Instruction::StoreIndirect { source, offset } => self.store_indirect(source, offset),
            Instruction::StoreRegister {
                source_1,
                source_2,
                offset,
            } => self.store_register(source_1, source_2, offset),
            Instruction::Noop => (),
        }
    }

    pub fn execute(&mut self) {
        self.running = true;
        while self.running {
            let pc = self.registers[Register::PC as usize] as usize;
            if pc == 0xFFFF {
                // We usually halt on a TRAP_HALT instruction, but implementation
                // of trap instructions is TODO
                self.running = false;
                break;
            }
            self.registers[Register::PC as usize] += 1;
            let instruction = Instruction::decode(self.memory[pc]);
            self.execute_instruction(instruction);
        }
    }

    fn update_flags(&mut self, value: u16) {
        // The wrapping add is necessary, because u16 doesn't know that a leading 1 indicates
        // a negative value
        self.registers[Register::Cond as usize] = match value.wrapping_add(1 << 15).cmp(&(1 << 15))
        {
            std::cmp::Ordering::Less => ConditionFlag::Negative as u16,
            std::cmp::Ordering::Equal => ConditionFlag::Zero as u16,
            std::cmp::Ordering::Greater => ConditionFlag::Positive as u16,
        }
    }

    fn add(&mut self, destination: Register, source_1: Register, source_2: Register) {
        let value =
            self.registers[source_1 as usize].wrapping_add(self.registers[source_2 as usize]);
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn add_immediate(&mut self, destination: Register, source: Register, mut value: u16) {
        value = value.wrapping_add(self.registers[source as usize]);
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn and(&mut self, destination: Register, source_1: Register, source_2: Register) {
        let value = self.registers[source_1 as usize] & self.registers[source_2 as usize];
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn and_immediate(&mut self, destination: Register, source: Register, mut value: u16) {
        value = value & self.registers[source as usize];
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn not(&mut self, destination: Register, source: Register) {
        let value = !(self.registers[source as usize]);
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn load(&mut self, destination: Register, offset: u16) {
        let address = self.registers[Register::PC as usize].wrapping_add(offset);
        let value = self.memory[address as usize];
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn load_register(&mut self, destination: Register, source: Register, offset: u16) {
        let address = self.registers[source as usize].wrapping_add(offset);
        let value = self.memory[address as usize];
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn load_indirect(&mut self, destination: Register, offset: u16) {
        let meta_address = self.registers[Register::PC as usize].wrapping_add(offset);
        let address = self.memory[meta_address as usize];
        let value = self.memory[address as usize];
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn load_effective_address(&mut self, destination: Register, offset: u16) {
        let value = self.registers[Register::PC as usize].wrapping_add(offset);
        self.registers[destination as usize] = value;
        self.update_flags(value);
    }

    fn branch(&mut self, flag: ConditionFlag, offset: u16) {
        if flag as u16 == self.registers[Register::Cond as usize] {
            self.registers[Register::PC as usize] =
                self.registers[Register::PC as usize].wrapping_add(offset);
        }
    }

    fn jump(&mut self, source: Register) {
        self.registers[Register::PC as usize] = self.registers[source as usize];
    }

    fn jump_immediate(&mut self, offset: u16) {
        self.registers[Register::R7 as usize] = self.registers[Register::PC as usize];
        let address = self.registers[Register::PC as usize].wrapping_add(offset);
        self.registers[Register::PC as usize] = address;
    }

    fn jump_register(&mut self, source: Register) {
        self.registers[Register::R7 as usize] = self.registers[Register::PC as usize];
        self.registers[Register::PC as usize] = self.registers[source as usize];
    }

    fn store(&mut self, source: Register, offset: u16) {
        let address: usize = offset.wrapping_add(self.registers[Register::PC as usize]) as usize;
        self.memory[address] = self.registers[source as usize];
    }

    fn store_indirect(&mut self, source: Register, offset: u16) {
        let meta_address = self.registers[Register::PC as usize].wrapping_add(offset) as usize;
        let address = self.memory[meta_address] as usize;
        self.memory[address] = self.registers[source as usize];
    }

    fn store_register(&mut self, source_1: Register, source_2: Register, offset: u16) {
        let address: usize = offset.wrapping_add(self.registers[source_2 as usize]) as usize;
        self.memory[address] = self.registers[source_1 as usize];
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::ConditionFlag;

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
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
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
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
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
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Negative as u16
        );
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
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Negative as u16
        );
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
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
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
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Zero as u16
        );
    }

    #[test]
    fn vm_and() {
        let instruction = Instruction::And {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 0,
            value: Register::R1 as u16,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R0 as usize] = 0xFFF0;
        vm.registers[Register::R1 as usize] = 0x0FFF;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 0x0FF0);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_and_immediate() {
        let instruction = Instruction::And {
            destination: Register::R0,
            source_1: Register::R0,
            source_2: Register::R1,
            mode: 1,
            value: 0xFFFF,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R0 as usize] = 0xFF00;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 0xFF00);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Negative as u16
        );
    }

    #[test]
    fn vm_not() {
        let instruction = Instruction::Not {
            destination: Register::R0,
            source: Register::R0,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R0 as usize] = 0xFF00;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 0x00FF);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_load() {
        let instruction = Instruction::Load {
            destination: Register::R0,
            offset: 3,
        };
        let mut vm = VirtualMachine::new();
        vm.memory[3] = 45;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 45);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_load_negative() {
        let instruction = Instruction::Load {
            destination: Register::R0,
            offset: 0xFFFD,
        };
        let mut vm = VirtualMachine::new();
        vm.memory[3] = 45;
        vm.registers[Register::PC as usize] = 6;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 45);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_load_indirect() {
        let instruction = Instruction::LoadIndirect {
            destination: Register::R0,
            offset: 3,
        };
        let mut vm = VirtualMachine::new();
        vm.memory[3] = 45;
        vm.memory[45] = 42;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 42);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_load_indirect_negative() {
        let instruction = Instruction::LoadIndirect {
            destination: Register::R0,
            offset: 0xFFFD,
        };
        let mut vm = VirtualMachine::new();
        vm.memory[3] = 45;
        vm.memory[45] = 42;
        vm.registers[Register::PC as usize] = 6;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 42);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_load_register() {
        let instruction = Instruction::LoadRegister {
            destination: Register::R0,
            source: Register::R0,
            offset: 3,
        };
        let mut vm = VirtualMachine::new();
        vm.memory[3] = 45;
        vm.memory[45] = 42;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 45);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_load_effective_address() {
        let instruction = Instruction::LoadEffectiveAddress {
            destination: Register::R0,
            offset: 3,
        };
        let mut vm = VirtualMachine::new();
        vm.memory[3] = 45;
        vm.memory[45] = 42;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::R0 as usize], 3);
        assert_eq!(
            vm.registers[Register::Cond as usize],
            ConditionFlag::Positive as u16
        );
    }

    #[test]
    fn vm_branch() {
        let instruction = Instruction::Branch {
            flag: ConditionFlag::Negative,
            offset: 16,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::Cond as usize] = ConditionFlag::Negative as u16;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::PC as usize], 16);
    }

    #[test]
    fn vm_branch_fail() {
        let instruction = Instruction::Branch {
            flag: ConditionFlag::Positive,
            offset: 16,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::PC as usize] = 3;
        vm.registers[Register::Cond as usize] = ConditionFlag::Negative as u16;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::PC as usize], 3);
    }

    #[test]
    fn vm_jump() {
        let instruction = Instruction::Jump {
            source: Register::R1,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::PC as usize] = 3;
        vm.registers[Register::R1 as usize] = 6;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::PC as usize], 6);
    }

    #[test]
    fn vm_jump_immediate() {
        let instruction = Instruction::JumpRegister {
            source: Register::R1,
            mode: 1,
            offset: 0x8FF,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::PC as usize] = 3;
        vm.registers[Register::R1 as usize] = 6;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::PC as usize], 0x8FF + 3);
    }

    #[test]
    fn vm_jump_register() {
        let instruction = Instruction::JumpRegister {
            source: Register::R1,
            mode: 0,
            offset: 0x40,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::PC as usize] = 3;
        vm.registers[Register::R1 as usize] = 6;
        vm.execute_instruction(instruction);
        assert_eq!(vm.registers[Register::PC as usize], 6);
    }

    #[test]
    fn vm_store() {
        let instruction = Instruction::Store {
            source: Register::R1,
            offset: 0x40,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 6;
        vm.execute_instruction(instruction);
        let address = (vm.registers[Register::PC as usize] + 0x40) as usize;
        assert_eq!(vm.memory[address], 6);
    }

    #[test]
    fn vm_store_indirect() {
        let instruction = Instruction::StoreIndirect {
            source: Register::R1,
            offset: 0x40,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 6;
        let address = (vm.registers[Register::PC as usize] + 0x40) as usize;
        vm.memory[address] = 50;
        vm.execute_instruction(instruction);
        assert_eq!(vm.memory[50], 6);
    }

    #[test]
    fn vm_store_register() {
        let instruction = Instruction::StoreRegister {
            source_1: Register::R1,
            source_2: Register::R2,
            offset: 0x40,
        };
        let mut vm = VirtualMachine::new();
        vm.registers[Register::R1 as usize] = 6;
        vm.registers[Register::R2 as usize] = 40;
        vm.execute_instruction(instruction);
        let address = (40 + 0x40) as usize;
        assert_eq!(vm.memory[address], 6);
    }

    #[test]
    fn vm_run_program() {
        let mut vm = VirtualMachine::new();
        let imm_add_1 = Instruction::Add {
            destination: Register::R1,
            source_1: Register::R0,
            source_2: Register::R0,
            mode: 1,
            value: 10,
        };
        let imm_add_2 = Instruction::Add {
            destination: Register::R2,
            source_1: Register::R0,
            source_2: Register::R0,
            mode: 1,
            value: 28,
        };
        let and = Instruction::And {
            destination: Register::R0,
            source_1: Register::R1,
            source_2: Register::R2,
            mode: 0,
            value: 2,
        };
        let store = Instruction::Store {
            source: Register::R0,
            offset: 4,
        };
        let load = Instruction::Load {
            destination: Register::R5,
            offset: 3,
        };
        let load_end = Instruction::Load {
            destination: Register::R4,
            offset: 1,
        };
        let jump = Instruction::Jump {
            source: Register::R4,
        };
        vm.memory[0] = imm_add_1.encode();
        vm.memory[1] = imm_add_2.encode();
        vm.memory[2] = and.encode();
        vm.memory[3] = store.encode();
        vm.memory[4] = load.encode();
        vm.memory[5] = load_end.encode();
        vm.memory[6] = jump.encode();
        vm.memory[7] = 0xFFFF;
        vm.execute();

        assert_eq!(vm.memory[8], 8);
        assert_eq!(vm.registers[Register::R5 as usize], 8);
        assert_eq!(vm.registers[Register::R4 as usize], 0xFFFF);
    }
}
