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
}