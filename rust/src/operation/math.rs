use crate::registers::Registers;
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

#[cfg(test)]
mod tests {
    mod add {
        use crate::operation::math::add;
        use crate::registers::{Address, Registers};

        #[test]
        fn given_immediate_flag_should_use_value() {
            let mut registers = Registers::new();
            registers.write_address(Address::R1, 1);
            let instruction = 0b0001000001100001;

            add(instruction, &mut registers);

            assert_eq!(2, registers.read_address(Address::R0));
        }

        #[test]
        fn given_no_immediate_flag_should_use_register() {
            let mut registers = Registers::new();
            registers.write_address(Address::R1, 1);
            registers.write_address(Address::R2, 1);
            let instruction = 0b0001000001000010;

            add(instruction, &mut registers);

            assert_eq!(2, registers.read_address(Address::R0));
        }
    }
}
