use crate::{
    hardware::{LC3, Register},
    isa::{ops, trap, Operation},
};

pub fn run(vm: &mut LC3) {
    let mut running = true;

    while running {
        let pc = vm.registers[Register::RPc as usize];
        let instr = vm.memory[pc as usize];
        vm.registers[Register::RPc as usize] = pc.wrapping_add(1);

        running = dispatch(vm, instr);
    }
}

fn dispatch(vm: &mut LC3, instr: u16) -> bool {
    let operation = instr >> 12;

    match operation {
        x if x == Operation::ADD as u16 => {
            ops::op_add(vm, instr);
            true
        }
        x if x == Operation::AND as u16 => {
            ops::op_and(vm, instr);
            true
        }
        x if x == Operation::NOT as u16 => {
            ops::op_not(vm, instr);
            true
        }
        x if x == Operation::BR as u16 => {
            ops::op_br(vm, instr);
            true
        }
        x if x == Operation::JMP as u16 => {
            ops::op_jmp(vm, instr);
            true
        }
        x if x == Operation::JSR as u16 => {
            ops::op_jsr(vm, instr);
            true
        }
        x if x == Operation::LD as u16 => {
            ops::op_ld(vm, instr);
            true
        }
        x if x == Operation::LDI as u16 => {
            ops::op_ldi(vm, instr);
            true
        }
        x if x == Operation::LDR as u16 => {
            ops::op_ldr(vm, instr);
            true
        }
        x if x == Operation::LEA as u16 => {
            ops::op_lea(vm, instr);
            true
        }
        x if x == Operation::ST as u16 => {
            ops::op_st(vm, instr);
            true
        }
        x if x == Operation::STI as u16 => {
            ops::op_sti(vm, instr);
            true
        }
        x if x == Operation::STR as u16 => {
            ops::op_str(vm, instr);
            true
        }
        x if x == Operation::TRAP as u16 => trap::op_trap(vm, instr),
        _ => {
            println!("operation doesnt exist: {:#X}", operation);
            false
        }
    }
}
