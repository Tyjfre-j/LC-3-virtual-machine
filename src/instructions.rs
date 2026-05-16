use crate::hardware::{LC3, Register};
/// LC-3 instruction set operations (opcodes)
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
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum ConditionFlag {
    /// Positive flag:
    POS = 1 << 0,

    /// Zero flag:
    ZRO = 1 << 1,

    /// Negative flag:
    NEG = 1 << 2,
}

pub fn sign_extend(x: u16, bit_count: u32) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFFu16.wrapping_shl(bit_count))
    } else {
        x
    }
}

pub fn update_flags(vm: &mut LC3, r: usize) {
    vm.registers[Register::RCond as usize] = if vm.registers[r] == 0 {
        ConditionFlag::ZRO as u16
    } else if vm.registers[r] >> 15 == 1 {
        ConditionFlag::NEG as u16
    } else {
        ConditionFlag::POS as u16
    };
}

pub fn op_add(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr   = ((instr >> 9) & 0x7) as usize;
    // first source register (SR1) is bits [8:6]
    let sr1  = ((instr >> 6) & 0x7) as usize;
    // mode bit is bit [5]: 0 for register mode, 1 for immediate mode
    let mode = (instr >> 5) & 0x1;

    vm.registers[dr] = if mode == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        vm.registers[sr1].wrapping_add(imm5)
    } else {
        let sr2 = (instr & 0x7) as usize;
        vm.registers[sr1].wrapping_add(vm.registers[sr2])
    };
    update_flags(vm, dr);
}

pub fn op_and(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr   = ((instr >> 9) & 0x7) as usize;
    // first source register (SR1) is bits [8:6]
    let sr1  = ((instr >> 6) & 0x7) as usize;
    // mode bit is bit [5]: 0 for register mode, 1 for immediate mode
    let mode = (instr >> 5) & 0x1;

    vm.registers[dr] = if mode == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        vm.registers[sr1] & imm5
    } else {
        let sr2 = (instr & 0x7) as usize;
        vm.registers[sr1] & vm.registers[sr2]
    };
    update_flags(vm, dr);
}

pub fn op_not(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr  = ((instr >> 9) & 0x7) as usize;
    // source register (SR1) is bits [8:6]
    let sr1 = ((instr >> 6) & 0x7) as usize;
    vm.registers[dr] = !vm.registers[sr1];
    update_flags(vm, dr);
}

pub fn op_br(vm: &mut LC3, instr: u16) {
    // PCoffset is bits [8:0], but we need to sign-extend it to 16 bits
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    // condition flags are bits [11:9]
    let cond_flag = (instr >> 9) & 0x7;
    // Check if any of the specified condition flags are set in the VM's RCond register
    if cond_flag & vm.registers[Register::RCond as usize] != 0 {
        vm.registers[Register::RPc as usize] =
            vm.registers[Register::RPc as usize].wrapping_add(pc_offset);
    }
}

pub fn op_jmp(vm: &mut LC3, instr: u16) {
    // also handles RET (which is just JMP with R7 as base)
    let r1 = ((instr >> 6) & 0x7) as usize;
    vm.registers[Register::RPc as usize] = vm.registers[r1];
}

pub fn op_jsr(vm: &mut LC3, instr: u16) {
    // long format: bit [11] = 1, JSR with PCoffset11
    let long_flag = (instr >> 11) & 1;
    vm.registers[Register::R7 as usize] = vm.registers[Register::RPc as usize];

    if long_flag == 1 {
        let offset = sign_extend(instr & 0x7FF, 11);
        vm.registers[Register::RPc as usize] =
            vm.registers[Register::RPc as usize].wrapping_add(offset);
    } else {
        let r1 = ((instr >> 6) & 0x7) as usize;
        vm.registers[Register::RPc as usize] = vm.registers[r1];
    }
}

pub fn op_ld(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr     = ((instr >> 9) & 0x7) as usize;
    // PCoffset is bits [8:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x1FF, 9);
    // the effective address is PC + offset
    let addr   = vm.registers[Register::RPc as usize].wrapping_add(offset);
    vm.registers[dr] = vm.memory[addr as usize];
    update_flags(vm, dr);
}

pub fn op_ldi(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr     = ((instr >> 9) & 0x7) as usize;
    // PCoffset is bits [8:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x1FF, 9);
    // the effective address is PC + offset, but we need to do an extra memory read to get the final address
    let ptr    = vm.registers[Register::RPc as usize].wrapping_add(offset);
    // first read: get the real address from memory, then read the value at that address
    let addr   = vm.memory[ptr as usize];       // first read: get the real address
    vm.registers[dr] = vm.memory[addr as usize]; // second read: get the value
    update_flags(vm, dr);
}

pub fn op_ldr(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr     = ((instr >> 9) & 0x7) as usize;
    // base register (BaseR) is bits [8:6]
    let base_r = ((instr >> 6) & 0x7) as usize;
    // offset is bits [5:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x3F, 6);
    // the effective address is BaseR + offset
    let addr   = vm.registers[base_r].wrapping_add(offset);
    vm.registers[dr] = vm.memory[addr as usize];
    update_flags(vm, dr);
}

pub fn op_lea(vm: &mut LC3, instr: u16) {
    // destination register (DR) is bits [11:9]
    let dr     = ((instr >> 9) & 0x7) as usize;
    // PCoffset is bits [8:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x1FF, 9);
    // loads the address itself, NOT the value at that address
    vm.registers[dr] = vm.registers[Register::RPc as usize].wrapping_add(offset);
    update_flags(vm, dr);
}

pub fn op_st(vm: &mut LC3, instr: u16) {
    // source register (SR) is bits [11:9]
    let sr     = ((instr >> 9) & 0x7) as usize;
    // PCoffset is bits [8:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x1FF, 9);
    // the effective address is PC + offset
    let addr   = vm.registers[Register::RPc as usize].wrapping_add(offset);
    vm.memory[addr as usize] = vm.registers[sr];
}

pub fn op_sti(vm: &mut LC3, instr: u16) {
    // source register (SR) is bits [11:9]
    let sr     = ((instr >> 9) & 0x7) as usize;
    // PCoffset is bits [8:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x1FF, 9);
    // the effective address is PC + offset but we need to do an extra memory read to get the final address
    let ptr    = vm.registers[Register::RPc as usize].wrapping_add(offset);
    // first read: get the real address from memory, then write the value to that address
    let addr   = vm.memory[ptr as usize];        
    vm.memory[addr as usize] = vm.registers[sr];
}

pub fn op_str(vm: &mut LC3, instr: u16) {
    // source register (SR) is bits [11:9]
    let sr     = ((instr >> 9) & 0x7) as usize;
    // base register (BaseR) is bits [8:6]
    let base_r = ((instr >> 6) & 0x7) as usize;
    // offset is bits [5:0], but we need to sign-extend it to 16 bits
    let offset = sign_extend(instr & 0x3F, 6);
    // the effective address is BaseR + offset
    let addr   = vm.registers[base_r].wrapping_add(offset);
    vm.memory[addr as usize] = vm.registers[sr];
}

pub fn op_trap(vm: &mut LC3, instr: u16, running: &mut bool) {
    // to be implemented next
}