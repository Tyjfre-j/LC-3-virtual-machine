/// LC-3 instruction set operations (opcodes)
///
/// Each operation corresponds to a 4-bit opcode in the LC-3 ISA.
/// These values are used during the decode stage of the CPU.
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum Operation {
    /// Branch operation: used for conditional branching based on condition flags (N, Z, P).
    BR = 0,

    /// Add operation: performs addition between two registers or one register and a value.
    ADD,

    /// Load operation: loads a value from memory into a register using PC-relative addressing.
    LD,

    /// Store operation: stores a value from a register into memory using PC (program counter)-relative addressing.
    ST,

    /// Jump to Subroutine: saves the current PC (program counter) and jumps to a subroutine address.
    JSR,

    /// AND operation: performs bitwise AND between two registers or one register and an immediate value.
    AND,

    /// Load Register: loads a value from memory using a base register + offset.
    LDR,

    /// Store Register: stores a value into memory using a base register + offset.
    STR,

    /// Return from Interrupt
    RTI,

    /// NOT operation: performs bitwise NOT on a register (one's complement).
    NOT,

    /// Load Indirect: loads a value from memory where the address is stored in memory.
    LDI,

    /// Store Indirect: stores a value into memory using an address stored in memory.
    STI,

    /// Jump: sets the PC (program counter) to the value in a register.
    JMP,

    /// Reserved:
    RES,

    /// Load Effective Address: loads the computed memory address (not the value at that address) into a register.
    LEA,

    /// Trap: executes a system call (OS service routine).
    TRAP,
}

/// LC-3 Condition Flags
#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum ConditionFlag {
    /// Positive flag:
    POS = 1 << 0,

    /// Zero flag:
    ZRO = 1 << 1,

    /// Negative flag:
    NEG = 1 << 2,
}