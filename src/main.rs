use instructions::{Instruction, TrapCode};
use terminal::{restore, setup};
use vm::VirtualMachine;

mod instructions;
mod terminal;
mod vm;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => "binaries/2048.obj".to_string(),
    };
    match VirtualMachine::from_image(path) {
        Ok(mut vm) => {
            let terminal = setup().unwrap(); // TODO: error handling
            vm.execute();
            restore(&terminal).unwrap();
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
