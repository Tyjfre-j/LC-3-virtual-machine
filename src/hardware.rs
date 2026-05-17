use std::fs::File;
use std::io::{self, Read};

use crate::terminal;
/// 64K memory locations, each storing a 16-bit value.
pub const MEMORY_SIZE: usize = 65536;

/// Memory-mapped registers
pub const MR_KBSR: u16 = 0xFE00; // keyboard status
pub const MR_KBDR: u16 = 0xFE02; // keyboard data

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

impl Default for LC3 {
    fn default() -> Self {
        Self::new()
    }
}

impl LC3 {
    /// Creates a new LC-3 instance.
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            registers: [0; Register::RCount as usize],
        }
    }

    /// Write to memory (includes memory-mapped registers).
    pub fn mem_write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    /// Read from memory (includes memory-mapped registers).
    pub fn mem_read(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            if terminal::check_key() {
                self.memory[MR_KBSR as usize] = 1 << 15;
                self.memory[MR_KBDR as usize] = terminal::get_char() as u16;
            } else {
                self.memory[MR_KBSR as usize] = 0;
            }
        }
        self.memory[address as usize]
    }

    /// Reads a binary image file into the LC-3 memory
    pub fn read_image(&mut self, path: &str) -> Result<(), io::Error> {
        let mut file = File::open(path)?;

        // The origin tells us where in memory to place the image (big-endian).
        let mut origin_bytes = [0u8; 2];
        file.read_exact(&mut origin_bytes)?;
        let origin = u16::from_be_bytes(origin_bytes) as usize;

        if origin >= MEMORY_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "origin is outside LC-3 memory",
            ));
        }

        // Max number of 16-bit words we can load without overflowing memory.
        let max_read_words = MEMORY_SIZE - origin;

        // Read words until EOF or we hit max_read_words.
        for i in 0..max_read_words {
            let mut word_bytes = [0u8; 2];
            match file.read_exact(&mut word_bytes) {
                Ok(()) => {
                    self.memory[origin + i] = u16::from_be_bytes(word_bytes);
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
