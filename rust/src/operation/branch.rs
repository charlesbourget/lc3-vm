use crate::registers::{Address, Registers};
use crate::utils::sign_extend;

/// Branch operation
///
/// | op |n|z|p|pc_offset|
pub fn branch(instruction: u16, registers: &mut Registers) {
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;

    if cond_flag & registers.read_address(Address::Cond) != 0 {
        registers.increment_pc(pc_offset);
    }
}

#[cfg(test)]
mod tests {
    mod branch {
        use crate::operation::branch::branch;
        use crate::registers::{Address, Registers};

        #[test]
        fn given_n_bit_match_should_increment_pc_with_pc_offset() {
            let mut registers = Registers::new();
            registers.write_address(Address::Cond, 1 << 2);
            registers.write_address(Address::PC, 2);
            let instruction = 0b0000_1_0_0_000000010;

            branch(instruction, &mut registers);

            assert_eq!(4, registers.read_address(Address::PC));
        }

        #[test]
        fn given_z_bit_match_should_increment_pc_with_pc_offset() {
            let mut registers = Registers::new();
            registers.write_address(Address::Cond, 1 << 1);
            registers.write_address(Address::PC, 2);
            let instruction = 0b0000_0_1_0_000000010;

            branch(instruction, &mut registers);

            assert_eq!(4, registers.read_address(Address::PC));
        }

        #[test]
        fn given_p_bit_match_should_increment_pc_with_pc_offset() {
            let mut registers = Registers::new();
            registers.write_address(Address::Cond, 1 << 0);
            registers.write_address(Address::PC, 2);
            let instruction = 0b0000_0_0_1_000000010;

            branch(instruction, &mut registers);

            assert_eq!(4, registers.read_address(Address::PC));
        }

        #[test]
        fn given_no_bit_match_should_not_increment_pc() {
            let mut registers = Registers::new();
            registers.write_address(Address::Cond, 1 << 0);
            registers.write_address(Address::PC, 2);
            let instruction = 0b0000_0_1_0_000000010;

            branch(instruction, &mut registers);

            assert_eq!(2, registers.read_address(Address::PC));
        }
    }
}
