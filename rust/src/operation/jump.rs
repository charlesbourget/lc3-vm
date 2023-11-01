use crate::registers::{Address, Registers};
use crate::utils::sign_extend;

/// Jump
///
/// |1100|000| r1|000000|
///
/// Return
///
/// |1100|000|111|000000|
pub fn jump(instruction: u16, registers: &mut Registers) {
    let r1 = (instruction >> 6) & 0x7;
    registers.write_address(Address::PC, registers.read(r1));
}

/// Jump to Subroutine
///
/// JSR
/// |0100|1|00000000000|
///
/// JSRR
/// |0100|0|00| r1|000000|
pub fn jump_sub_routine(instruction: u16, registers: &mut Registers) {
    let long_flag = (instruction >> 11) & 0x1;
    registers.write_address(Address::R7, registers.read_address(Address::PC));

    if long_flag == 1 {
        let long_pc_offset = sign_extend(instruction & 0x7FF, 11);
        registers.increment_pc(long_pc_offset);
    } else {
        let r1 = (instruction >> 6) & 0x7;
        registers.increment_pc(registers.read(r1));
    }
}

#[cfg(test)]
mod tests {
    mod jump {
        use crate::operation::jump::jump;
        use crate::registers::{Address, Registers};

        #[test]
        fn set_pc_to_r1_value() {
            let mut registers = Registers::new();
            registers.write_address(Address::R1, 6);
            let instruction = 0b1100_000_001_000000;

            jump(instruction, &mut registers);

            assert_eq!(6, registers.read_address(Address::PC));
        }
    }
}
