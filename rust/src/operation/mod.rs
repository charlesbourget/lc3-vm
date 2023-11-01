use crate::operation::bitwise::{and, not};
use crate::operation::branch::branch;
use crate::operation::jump::{jump, jump_sub_routine};
use crate::operation::load::{load, load_effective_address, load_indirect, load_register};
use crate::operation::math::add;
use crate::operation::trap::trap;
use crate::vm::{Memory, Registers};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

mod bitwise;
mod branch;
mod jump;
mod load;
mod math;
mod trap;

#[derive(FromPrimitive)]
pub enum Operation {
    Branch,
    Add,
    Load,
    Store,
    JumpSubRoutine,
    And,
    LoadRegister,
    StoreRegister,
    ReturnFromInterrupt, /* unused */
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    Reserved, /* unused */
    LoadEffectiveAddress,
    Trap,
}

pub fn execute_instruction(
    instruction: u16,
    registers: &mut Registers,
    memory: &mut Memory,
) -> bool {
    let operation: Operation = FromPrimitive::from_u16(instruction >> 12).unwrap();

    let mut running = true;

    match operation {
        Operation::Add => add(instruction, registers),
        Operation::And => and(instruction, registers),
        Operation::Not => not(instruction, registers),
        Operation::Branch => branch(instruction, registers),
        Operation::Jump => jump(instruction, registers),
        Operation::JumpSubRoutine => jump_sub_routine(instruction, registers),
        Operation::Load => load(instruction, registers, memory),
        Operation::LoadIndirect => load_indirect(),
        Operation::LoadRegister => load_register(),
        Operation::LoadEffectiveAddress => load_effective_address(instruction, registers),
        Operation::Store => todo!(),
        Operation::StoreIndirect => todo!(),
        Operation::StoreRegister => todo!(),
        Operation::ReturnFromInterrupt => todo!(),
        Operation::Reserved => todo!(),
        Operation::Trap => {
            if let Some(v) = trap(instruction, registers, memory) {
                running = v;
            }
        }
    }

    running
}
