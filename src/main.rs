use cpu::CPU;


pub mod cpu;
pub mod registers;
pub mod lib;
pub mod instructions;
pub mod memory;

fn main() {
    let mut cpu = CPU::default();
    // cpu.load(program);
    cpu.reset();
    cpu.run()
}
