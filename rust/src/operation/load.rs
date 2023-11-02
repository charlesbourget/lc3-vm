use crate::utils::sign_extend;
use crate::vm::Vm;
use anyhow::Error;

pub fn load_effective_address(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    vm.registers.write(r0, vm.registers.pc + pc_offset)?;

    Ok(())
}

pub fn load(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    vm.registers
        .write(r0, vm.memory.read(vm.registers.pc + pc_offset))?;
    vm.registers.update_flags(r0)?;

    Ok(())
}

pub fn load_register(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);
    let value = vm.memory.read(vm.registers.read(r1)? + offset);

    vm.registers.write(r0, value)?;
    vm.registers.update_flags(r0)?;

    Ok(())
}

pub fn load_indirect(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    vm.registers.r0 = vm.memory.read(vm.memory.read(vm.registers.pc + pc_offset));
    vm.registers.update_flags(r0)?;

    Ok(())
}
