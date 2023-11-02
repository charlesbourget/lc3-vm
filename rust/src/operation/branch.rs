use crate::utils::sign_extend;
use crate::vm::Vm;

/// Branch operation
///
/// | op |n|z|p|pc_offset|
pub fn branch(instruction: u16, vm: &mut Vm) {
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;

    if cond_flag & vm.registers.cond != 0 {
        vm.registers.pc += pc_offset;
    }
}

#[cfg(test)]
mod tests {
    mod branch {
        use crate::operation::branch::branch;
        use crate::vm::Vm;

        #[test]
        fn given_n_bit_match_should_increment_pc_with_pc_offset() {
            let mut vm = Vm::new();
            vm.registers.cond = 1 << 2;
            vm.registers.pc = 2;
            let instruction = 0b0000_1_0_0_000000010;

            branch(instruction, &mut vm);

            assert_eq!(4, vm.registers.pc);
        }

        #[test]
        fn given_z_bit_match_should_increment_pc_with_pc_offset() {
            let mut vm = Vm::new();
            vm.registers.cond = 1 << 1;
            vm.registers.pc = 2;
            let instruction = 0b0000_0_1_0_000000010;

            branch(instruction, &mut vm);

            assert_eq!(4, vm.registers.pc);
        }

        #[test]
        fn given_p_bit_match_should_increment_pc_with_pc_offset() {
            let mut vm = Vm::new();
            vm.registers.cond = 1 << 0;
            vm.registers.pc = 2;
            let instruction = 0b0000_0_0_1_000000010;

            branch(instruction, &mut vm);

            assert_eq!(4, vm.registers.pc);
        }

        #[test]
        fn given_no_bit_match_should_not_increment_pc() {
            let mut vm = Vm::new();
            vm.registers.cond = 1 << 0;
            vm.registers.pc = 2;
            let instruction = 0b0000_0_1_0_000000010;

            branch(instruction, &mut vm);

            assert_eq!(2, vm.registers.pc);
        }
    }
}
