use crate::utils::sign_extend;
use crate::vm::Vm;
use anyhow::Error;

/// And operation
///
/// immediate mode
/// |0101| r0| r1|1| imm5|
///
/// indirect mode
/// |0101| r0| r1|0|00| r2|
pub fn and(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let immediate_flag = (instruction >> 5) & 0x1;

    let value = if immediate_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);

        vm.registers.read(r1)? & imm5
    } else {
        let r2 = instruction & 0x7;

        vm.registers.read(r1)? & vm.registers.read(r2)?
    };

    vm.registers.write(r0, value)?;
    vm.registers.update_flags(r0)?;

    Ok(())
}

/// Not operation
///
/// |1001| r0| r1|1|11111|
pub fn not(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;

    vm.registers.write(r0, !vm.registers.read(r1)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    mod and {
        use crate::operation::bitwise::and;
        use crate::vm::Vm;

        #[test]
        fn given_immediate_flag_should_use_value() {
            let mut vm = Vm::new();
            vm.registers.r1 = 6;
            let instruction = 0b0101_000_001_1_00101;

            and(instruction, &mut vm).unwrap();

            assert_eq!(4, vm.registers.r0);
        }

        #[test]
        fn given_no_immediate_flag_should_use_register() {
            let mut vm = Vm::new();
            vm.registers.r1 = 5;
            vm.registers.r2 = 6;
            let instruction = 0b0101_000_001_0_00_010;

            and(instruction, &mut vm).unwrap();

            assert_eq!(4, vm.registers.r0);
        }
    }

    mod not {
        use crate::operation::bitwise::not;
        use crate::vm::Vm;

        #[test]
        fn negate_register_value() {
            let mut vm = Vm::new();
            vm.registers.r1 = 5;
            let instruction = 0b1001_000_001_000000;

            not(instruction, &mut vm).unwrap();

            assert_eq!(65530, vm.registers.r0);
        }
    }
}
