use crate::operation::bitwise::{and, not};
use crate::operation::branch::branch;
use crate::operation::jump::{jump, jump_sub_routine};
use crate::operation::load::{load, load_effective_address, load_indirect, load_register};
use crate::operation::math::add;
use crate::operation::store::{store, store_indirect, store_register};
use crate::operation::trap::trap;
use crate::vm::Vm;
use anyhow::{anyhow, Context, Error};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

mod bitwise;
mod branch;
mod jump;
mod load;
mod math;
mod store;
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

pub fn execute_instruction(instruction: u16, vm: &mut Vm) -> Result<bool, Error> {
    let operation: Operation = FromPrimitive::from_u16(instruction >> 12)
        .with_context(|| format!("Invalid op code encountered {}", instruction & 0xFF))?;

    let mut running = true;

    match operation {
        Operation::Add => add(instruction, vm)?,
        Operation::And => and(instruction, vm)?,
        Operation::Not => not(instruction, vm)?,
        Operation::Branch => branch(instruction, vm),
        Operation::Jump => jump(instruction, vm)?,
        Operation::JumpSubRoutine => jump_sub_routine(instruction, vm)?,
        Operation::Load => load(instruction, vm)?,
        Operation::LoadIndirect => load_indirect(instruction, vm)?,
        Operation::LoadRegister => load_register(instruction, vm)?,
        Operation::LoadEffectiveAddress => load_effective_address(instruction, vm)?,
        Operation::Store => store(instruction, vm)?,
        Operation::StoreIndirect => store_indirect(instruction, vm)?,
        Operation::StoreRegister => store_register(instruction, vm)?,
        Operation::ReturnFromInterrupt => {
            return Err(anyhow!("Illegal op code [ReturnFromInterrupt] used."))
        }
        Operation::Reserved => return Err(anyhow!("Illegal op code [Reserved] used.")),
        Operation::Trap => {
            running = trap(instruction, vm)?;
        }
    }

    Ok(running)
}
