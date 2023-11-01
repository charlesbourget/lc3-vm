#![feature(array_chunks)]

mod operation;
mod utils;
mod vm;

use crate::vm::{Address, Flag, Memory, Vm};
use std::{env, fs, io, process::exit};

fn main() {
    let mut vm = Vm::new();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("lc3 [image-file] ...");

        exit(2);
    }

    for image_file in args.iter().skip(1) {
        if read_image_file(image_file, &mut vm.memory).is_err() {
            panic!("failed to load image: {:?}", image_file);
        }
    }

    // TODO: Handle interrupt signals
    // TODO: disable_input_buffering, do we need this?

    vm.registers
        .write_condition_flag_address(Address::Cond, Flag::Zero);
    // default start position
    vm.registers.write_address(Address::PC, 0x3000);

    let mut running = true;
    while running {
        // Increment PC
        let pc = vm.registers.read_address(Address::PC) + 1;
        vm.registers.write_address(Address::PC, pc);
        // Load operation
        let instruction = vm.memory.read(pc);
        // Execute operation
        running = operation::execute_instruction(instruction, &mut vm.registers, &mut vm.memory);
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
