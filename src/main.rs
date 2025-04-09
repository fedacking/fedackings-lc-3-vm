use instructions::{Instruction, TrapCode};
use vm::VirtualMachine;

mod instructions;
mod vm;

fn main() {
    let mut vm = VirtualMachine::from_image("binaries/2048.obj".to_string()).unwrap();
    vm.execute();
}
