use anyhow::{anyhow, Error};
use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Address {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    Cond,
    Count,
}

pub enum Flag {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

pub enum _MemoryMappedRegisters {
    Kbsr = 0xFE00,
    Kbdr = 0xFE02,
}

const MEMORY_MAX: usize = 1 << 16;

pub struct Vm {
    pub memory: Memory,
    pub registers: Registers,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }
}

pub struct Memory {
    values: [u16; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            values: [0; MEMORY_MAX],
        }
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.values[address as usize] = value;
    }

    pub fn write_chunk(&mut self, address: u16, values: &[u16]) {
        for (i, value) in values.iter().enumerate() {
            self.values[(address + i as u16) as usize] = *value;
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        self.values[address as usize]
    }
}

pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
    pub count: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0,
            cond: 0,
            count: 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u16) -> Result<(), Error> {
        match address {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.pc = value,
            9 => self.cond = value,
            10 => self.count = value,
            _ => return Err(anyhow!(format!("Invalid register address {}", address))),
        }

        Ok(())
    }

    pub fn read(&self, address: u16) -> Result<u16, Error> {
        let value = match address {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            10 => self.count,
            _ => return Err(anyhow!(format!("Invalid register address {}", address))),
        };

        Ok(value)
    }

    pub fn update_flags(&mut self, address: u16) -> Result<(), Error> {
        let value = self.read(address)?;
        let mut flag = Flag::Positive;

        if value == 0 {
            flag = Flag::Zero;
        } else if value >> 15 == 1 {
            flag = Flag::Negative;
        }

        self.cond = flag as u16;

        Ok(())
    }
}
