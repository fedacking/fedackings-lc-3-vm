use vm::VirtualMachine;

mod constants;
mod vm;

fn main() {
    let main_vm = VirtualMachine::new();
    println!("Hello, world!");
}
