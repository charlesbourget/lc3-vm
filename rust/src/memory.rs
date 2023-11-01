const MEMORY_MAX: usize = 1 << 16;

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
