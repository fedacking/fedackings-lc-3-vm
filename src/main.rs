use instructions::VMError;
use terminal::{restore, setup};
use vm::VirtualMachine;

mod instructions;
mod terminal;
mod vm;

fn launch_vm(path: String) -> Result<(), VMError> {
    let mut vm = VirtualMachine::from_image(path)?;
    let terminal = setup().map_err(|err| VMError::IO { err })?;
    vm.execute()?;
    restore(&terminal).map_err(|err| VMError::IO { err })?;
    Ok(())
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => "binaries/2048.obj".to_string(),
    };
    match launch_vm(path) {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    };
}
