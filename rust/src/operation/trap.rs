use crate::vm::{Address, Memory, Registers};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum TrapCode {
    GetChar = 0x20,
    Out = 0x21,
    PutS = 0x22,
    In = 0x23,
    PutSP = 0x24,
    Halt = 0x25,
}

pub fn trap(instruction: u16, registers: &mut Registers, memory: &Memory) -> Option<bool> {
    registers.write_address(Address::R7, registers.read_address(Address::PC));
    let trap_code: TrapCode = FromPrimitive::from_u16(instruction & 0xFF).unwrap();

    match trap_code {
        TrapCode::GetChar => todo!(),
        TrapCode::Out => todo!(),
        TrapCode::PutS => puts(registers, memory),
        TrapCode::In => todo!(),
        TrapCode::PutSP => todo!(),
        TrapCode::Halt => {
            println!("Halt");
            return Some(false);
        }
    }

    None
}

fn puts(registers: &Registers, memory: &Memory) {
    let string_start_address = registers.read_address(Address::R0);

    let mut current_address = string_start_address + 1;
    let mut character = memory.read(current_address);
    while character != 0x0000 {
        print!("{}", character as u8 as char);
        current_address += 1;
        character = memory.read(current_address)
    }
}
