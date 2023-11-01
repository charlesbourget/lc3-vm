use crate::utils::sign_extend;
use crate::vm::Registers;

/// And operation
///
/// immediate mode
/// |0101| r0| r1|1| imm5|
///
/// indirect mode
/// |0101| r0| r1|0|00| r2|
pub fn and(instruction: u16, registers: &mut Registers) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let immediate_flag = (instruction >> 5) & 0x1;

    let value = if immediate_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);

        registers.read(r1) & imm5
    } else {
        let r2 = instruction & 0x7;

        registers.read(r1) & registers.read(r2)
    };

    registers.write(r0, value);
    registers.update_flags(r0);
}

/// Not operation
///
/// |1001| r0| r1|1|11111|
pub fn not(instruction: u16, registers: &mut Registers) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;

    registers.write(r0, !registers.read(r1));
}

#[cfg(test)]
mod tests {
    mod and {
        use crate::operation::bitwise::and;
        use crate::vm::{Address, Registers};

        #[test]
        fn given_immediate_flag_should_use_value() {
            let mut registers = Registers::new();
            registers.write_address(Address::R1, 6);
            let instruction = 0b0101_000_001_1_00101;

            and(instruction, &mut registers);

            assert_eq!(4, registers.read_address(Address::R0));
        }

        #[test]
        fn given_no_immediate_flag_should_use_register() {
            let mut registers = Registers::new();
            registers.write_address(Address::R1, 5);
            registers.write_address(Address::R2, 6);
            let instruction = 0b0101_000_001_0_00_010;

            and(instruction, &mut registers);

            assert_eq!(4, registers.read_address(Address::R0));
        }
    }

    mod not {
        use crate::operation::bitwise::not;
        use crate::vm::{Address, Registers};

        #[test]
        fn negate_register_value() {
            let mut registers = Registers::new();
            registers.write_address(Address::R1, 5);
            let instruction = 0b1001_000_001_000000;

            not(instruction, &mut registers);

            assert_eq!(65530, registers.read_address(Address::R0));
        }
    }
}
