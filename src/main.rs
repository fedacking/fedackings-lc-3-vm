use instructions::{Instruction, TrapCode};
use vm::VirtualMachine;

mod instructions;
mod vm;
mod terminal;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => "binaries/2048.obj".to_string(),
    };
    match VirtualMachine::from_image(path) {
        Ok(mut vm) => {
            vm.execute();
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
