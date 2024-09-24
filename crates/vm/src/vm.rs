pub mod memory;
pub mod op;

use crate::memory::Memory;

pub struct AstraMachine {
    registers: Vec<u16>,
    memory: Memory,
}

impl AstraMachine {
    pub fn new() -> Self {
        Self {
            registers: Vec::new(),
            memory: Memory::new(10 * 1024),
        }
    }

    pub fn step() {}
}
