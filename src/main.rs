use vm::VirtualMachine;

mod instructions;
mod vm;

fn main() {
    let main_vm = VirtualMachine::new();
    println!("Hello, world!");
}
