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

    pub fn _write(&mut self, address: u16, value: u16) {
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
    values: [u16; 11],
}

impl Registers {
    pub fn new() -> Self {
        Registers { values: [0; 11] }
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.values[address as usize] = value;
    }

    pub fn write_address(&mut self, address: Address, value: u16) {
        self.write(address as u16, value);
    }

    pub fn write_condition_flag_address(&mut self, address: Address, value: Flag) {
        self.write_address(address, value as u16);
    }

    pub fn increment_pc(&mut self, increment_value: u16) {
        self.write_address(
            Address::PC,
            self.read_address(Address::PC) + increment_value,
        );
    }

    pub fn read(&self, address: u16) -> u16 {
        self.values[address as usize]
    }

    pub fn read_address(&self, address: Address) -> u16 {
        self.read(address as u16)
    }

    pub fn update_flags(&mut self, address: u16) {
        let value = self.read(address);
        let mut flag = Flag::Positive;

        if value == 0 {
            flag = Flag::Zero;
        } else if value >> 15 == 1 {
            flag = Flag::Negative;
        }

        self.write_condition_flag_address(Address::Cond, flag);
    }
}
