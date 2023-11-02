use crate::utils::sign_extend;
use crate::vm::Vm;
use anyhow::Error;

pub fn store(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    vm.memory
        .write(vm.registers.pc + pc_offset, vm.registers.read(r0)?);

    Ok(())
}

pub fn store_indirect(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    vm.memory.write(
        vm.memory.read(vm.registers.pc + pc_offset),
        vm.registers.read(r0)?,
    );

    Ok(())
}

pub fn store_register(instruction: u16, vm: &mut Vm) -> Result<(), Error> {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    vm.memory
        .write(vm.registers.read(r1)? + offset, vm.registers.read(r0)?);

    Ok(())
}
