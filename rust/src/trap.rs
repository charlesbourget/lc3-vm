use crate::memory::Memory;
use crate::registers::{Address, Registers};

pub fn puts(registers: &Registers, memory: &Memory) {
    let string_start_address = registers.read_address(Address::R0);

    let mut current_address = string_start_address + 1;
    let mut character = memory.read(current_address);
    while character != 0x0000 {
        print!("{}", character as u8 as char);
        current_address += 1;
        character = memory.read(current_address)
    }
}