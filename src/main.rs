use instructions::{Instruction, TrapCode};
use vm::VirtualMachine;

mod instructions;
mod vm;

fn main() {
    let mut bloc = [0; u16::MAX as usize];
    bloc[0x3000] = Instruction::Trap {
        routine: TrapCode::TrapIn,
    }
    .encode();
    bloc[0x3000 + 1] = Instruction::Trap {
        routine: TrapCode::TrapHalt,
    }
    .encode();
    let mut vm = VirtualMachine::from_program(bloc);
    vm.execute();
}
