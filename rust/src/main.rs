#![feature(array_chunks)]
mod codes;
mod memory;
mod registers;
mod operations;
mod utils;
mod trap;

use crate::memory::Memory;
use crate::registers::{Address, Registers};
use num_traits::FromPrimitive;
use std::{env, fs, io, process::exit};

fn main() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("lc3 [image-file] ...");

        exit(2);
    }

    for image_file in args.iter().skip(1) {
        if read_image_file(image_file, &mut memory).is_err() {
            panic!("failed to load image: {:?}", image_file);
        }
    }

    // TODO: Handle interrupt signals
    // TODO: disable_input_buffering, do we need this?

    registers.write_condition_flag_address(Address::Cond, registers::Flag::Zero);
    // default start position
    registers.write_address(Address::PC, 0x3000);

    let mut running = true;
    while running {
        // Increment PC
        let pc = registers.read_address(Address::PC) + 1;
        registers.write_address(Address::PC, pc);
        // Load instruction
        let instruction = memory.read(pc);
        let operation: codes::Operation = FromPrimitive::from_u16(instruction >> 12).unwrap();

        match operation {
            codes::Operation::Add => operations::add(instruction, &mut registers),
            codes::Operation::And => todo!(),
            codes::Operation::Not => todo!(),
            codes::Operation::Branch => todo!(),
            codes::Operation::Jump => todo!(),
            codes::Operation::JumpRegister => todo!(),
            codes::Operation::Load => todo!(),
            codes::Operation::LoadIndirect => todo!(),
            codes::Operation::LoadRegister => todo!(),
            codes::Operation::LoadEffectiveAddress => operations::load_effective_address(instruction, &mut registers),
            codes::Operation::Store => todo!(),
            codes::Operation::StoreIndirect => todo!(),
            codes::Operation::StoreRegister => todo!(),
            codes::Operation::ReturnFromInterrupt => todo!(),
            codes::Operation::Reserved => todo!(),
            codes::Operation::Trap => {
                if let Some(v) = operations::trap(instruction, &mut registers, &memory) {
                    running = v
                }
            }
        }
    }

    // TODO: restore input buffering, do we need this?
}

fn read_image_file(image_file: &str, memory: &mut Memory) -> Result<(), io::Error> {
    let data: Vec<u16> = fs::read(image_file)?
        .array_chunks()
        .map(|x| u16::from_be_bytes(*x))
        .collect();

    let origin = data[0];
    memory.write_chunk(origin + 1, &data[1..]);

    Ok(())
}
