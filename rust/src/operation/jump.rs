use crate::utils::sign_extend;
use crate::vm::Vm;
use anyhow::Error;

/// Jump
///
/// |1100|000| r1|000000|
///
/// Return
///
/// |1100|000|111|000000|
pub fn jump(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r1 = (instruction >> 6) & 0x7;
    vm.registers.pc = vm.registers.read(r1)?;

    Ok(())
}

/// Jump to Subroutine
///
/// JSR
/// |0100|1|00000000000|
///
/// JSRR
/// |0100|0|00| r1|000000|
pub fn jump_sub_routine(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let long_flag = (instruction >> 11) & 0x1;
    vm.registers.r7 = vm.registers.pc;

    if long_flag == 1 {
        let long_pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers.pc += long_pc_offset;
    } else {
        let r1 = (instruction >> 6) & 0x7;
        vm.registers.pc += vm.registers.read(r1)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    mod jump {
        use crate::operation::jump::jump;
        use crate::vm::Vm;

        #[test]
        fn set_pc_to_r1_value() {
            let mut vm = Vm::new();
            vm.registers.r1 = 6;
            let instruction = 0b1100_000_001_000000;

            jump(instruction, &mut vm).unwrap();

            assert_eq!(6, vm.registers.pc);
        }
    }
}
