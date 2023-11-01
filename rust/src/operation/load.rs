use crate::utils::sign_extend;
use crate::vm::{Address, Memory, Registers};

pub fn load_effective_address(instruction: u16, registers: &mut Registers) {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let value = registers.read_address(Address::PC) + pc_offset;
    registers.write(r0, value);
}

pub fn load(instruction: u16, registers: &mut Registers, memory: &Memory) {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let memory_address = registers.read_address(Address::PC) + pc_offset;
    registers.write(r0, memory.read(memory_address));

    registers.update_flags(r0);
}

pub fn load_register() {
    todo!();
}

pub fn load_indirect() {
    todo!();
}
