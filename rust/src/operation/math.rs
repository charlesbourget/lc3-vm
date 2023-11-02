use crate::utils::sign_extend;
use crate::vm::Vm;
use anyhow::Error;

pub fn add(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let immediate_flag = (instruction >> 5) & 0x1;

    if immediate_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        let value = vm.registers.read(r1)? + imm5;
        vm.registers.write(r0, value)?;
    } else {
        let r2 = instruction & 0x7;
        let value = vm.registers.read(r1)? + vm.registers.read(r2)?;
        vm.registers.write(r0, value)?;
    }

    vm.registers.update_flags(r0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    mod add {
        use crate::operation::math::add;
        use crate::vm::Vm;

        #[test]
        fn given_immediate_flag_should_use_value() {
            let mut vm = Vm::new();
            vm.registers.r1 = 1;
            let instruction = 0b0001000001100001;

            add(instruction, &mut vm).unwrap();

            assert_eq!(2, vm.registers.r0);
        }

        #[test]
        fn given_no_immediate_flag_should_use_register() {
            let mut vm = Vm::new();
            vm.registers.r1 = 1;
            vm.registers.r2 = 1;
            let instruction = 0b0001000001000010;

            add(instruction, &mut vm).unwrap();

            assert_eq!(2, vm.registers.r0);
        }
    }
}
