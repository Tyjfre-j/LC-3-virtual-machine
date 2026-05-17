use std::io::Write;

use crate::hardware::{LC3, Register};
use crate::terminal;

use super::{types::TrapCode, utils::update_flags};

pub fn op_trap(vm: &mut LC3, instr: u16) -> bool {
    vm.registers[Register::R7 as usize] = vm.registers[Register::RPc as usize];

    match instr & 0xFF {
        x if x == TrapCode::GetC as u16 => trap_getc(vm),
        x if x == TrapCode::Out as u16 => trap_out(vm),
        x if x == TrapCode::Puts as u16 => trap_puts(vm),
        x if x == TrapCode::In as u16 => trap_in(vm),
        x if x == TrapCode::PutsP as u16 => trap_putsp(vm),
        x if x == TrapCode::Halt as u16 => {
            println!("HALT");
            return false;
        }
        _ => {}
    }
    true
}

fn trap_getc(vm: &mut LC3) {
    let c = terminal::get_char() as u16;
    vm.registers[Register::R0 as usize] = c;
    update_flags(vm, Register::R0 as usize);
}

fn trap_out(vm: &mut LC3) {
    let c = vm.registers[Register::R0 as usize] as u8;
    print!("{}", c as char);
    let _ = std::io::stdout().flush();
}

fn trap_puts(vm: &mut LC3) {
    let mut addr = vm.registers[Register::R0 as usize];
    loop {
        let c = vm.mem_read(addr);
        if c == 0 {
            break;
        }
        print!("{}", c as u8 as char);
        addr = addr.wrapping_add(1);
    }
    let _ = std::io::stdout().flush();
}

fn trap_in(vm: &mut LC3) {
    print!("Enter a character: ");
    let _ = std::io::stdout().flush();
    let c = terminal::get_char();
    print!("{}", c as char);
    let _ = std::io::stdout().flush();
    vm.registers[Register::R0 as usize] = c as u16;
    update_flags(vm, Register::R0 as usize);
}

fn trap_putsp(vm: &mut LC3) {
    let mut addr = vm.registers[Register::R0 as usize];
    loop {
        let word = vm.mem_read(addr);
        if word == 0 {
            break;
        }
        let c1 = (word & 0xFF) as u8;
        print!("{}", c1 as char);
        let c2 = (word >> 8) as u8;
        if c2 != 0 {
            print!("{}", c2 as char);
        }
        addr = addr.wrapping_add(1);
    }
    let _ = std::io::stdout().flush();
}
