use std::fs::File;
use std::io::{self, Read};
/// 64K memory locations, each storing a 16-bit value.
pub const MEMORY_SIZE: usize = 65536;

/// LC-3 CPU Registers
#[derive(Copy, Clone)]
#[repr(usize)] 
pub enum Register {
    /// R0 to R7: General-purpose registers used for data operations.
    R0 = 0,
    /// R0 to R7: General-purpose registers used for data operations.
    R1,
    /// R0 to R7: General-purpose registers used for data operations.
    R2,
    /// R0 to R7: General-purpose registers used for data operations.
    R3,
    /// R0 to R7: General-purpose registers used for data operations.
    R4,
    /// R0 to R7: General-purpose registers used for data operations.
    R5,
    /// R0 to R7: General-purpose registers used for data operations.
    R6,
    /// R0 to R7: General-purpose registers used for data operations.
    R7,
    /// Program Counter: Stores the memory address of the next instruction.
    RPc,
    /// Condition Flags: Tracks if the last result was Negative, Zero, or Positive.
    RCond,
    /// number of registers ( not a real register )
    RCount,
}

/// The LC-3 Virtual Machine State
pub struct LC3 {
    /// virtual machine memory.
    pub memory: [u16; MEMORY_SIZE],
    /// virtual machine registers.
    pub registers: [u16; Register::RCount as usize],
}

impl LC3 {
    /// Creates a new LC-3 instance.
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            registers: [0; Register::RCount as usize],
        }
    }

    /// Reads a binary image file into the LC-3 memory
    pub fn read_image(&mut self, path: &str) -> Result<(), io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let origin = u16::from_be_bytes([buffer[0], buffer[1]]) as usize;

        let mut addr = origin;
        let mut i = 2;
        while i + 1 < buffer.len() {
            self.memory[addr] = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
            addr += 1;
            i += 2;
        }
        Ok(())
    }
}