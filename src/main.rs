extern crate x86emulator_rs;

use x86emulator_rs::emulator::Emulator;

enum registers_name {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

const MEMORY_SIZE: i32 = 1024 * 1024;


fn read_binary(emu: &Emulator, filename: &str) {}

fn dump_registers(emu: &Emulator) {}


fn create_emu(size: usize, eip: u32, esp: u32) -> Emulator {}


fn destroy_emu(emu: &Emulator) {}



fn main() {
    println!("Hello, world!");
}
