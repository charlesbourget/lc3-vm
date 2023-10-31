use num_traits::FromPrimitive;
use crate::codes::TrapCode;
use crate::memory::Memory;
use crate::registers::{Address, Registers};
use crate::trap;
use crate::utils::sign_extend;

pub fn add(instruction: u16, registers: &mut Registers) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let immediate_flag = (instruction >> 5) & 0x1;

    if immediate_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        let value = registers.read(r1) + imm5;
        registers.write(r0, value);
    } else {
        let r2 = instruction & 0x7;
        let value = registers.read(r1) + registers.read(r2);
        registers.write(r0, value);
    }

    registers.update_flags(r0);
}

pub fn load_effective_address(instruction: u16, registers: &mut Registers) {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let value = registers.read_address(Address::PC) + pc_offset;
    registers.write(r0, value);
}

pub fn trap(instruction: u16, registers: &mut Registers, memory: &Memory) -> Option<bool> {
    registers.write_address(Address::R7, registers.read_address(Address::PC));
    let trap_code: TrapCode = FromPrimitive::from_u16(instruction & 0xFF).unwrap();

    match trap_code {
        TrapCode::GetChar => todo!(),
        TrapCode::Out => todo!(),
        TrapCode::PutS => trap::puts(registers, memory),
        TrapCode::In => todo!(),
        TrapCode::PutSP => todo!(),
        TrapCode::Halt => {
            println!("Halt");
            return Some(false);
        }
    }

    None
}