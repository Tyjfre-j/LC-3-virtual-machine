mod hardware;
mod instructions;

use hardware::{LC3, Register};
use instructions::*;
fn main() {
    // Handle CLI arguments (equivalent to argc/argv check)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("lc3 [image-file1] , [image-file2] ... this is how you load programs into the virtual machine");
        std::process::exit(2);
    }
    // Initialize the LC-3 virtual machine
    let mut virtual_machine = LC3::new();
    virtual_machine.registers[Register::RCond as usize] = ConditionFlag::ZRO as u16;
    virtual_machine.registers[Register::RPc as usize] = 0x3000;
    
    // Load image files into VM memory
    for image_path in &args[1..] {
        if let Err(_) = virtual_machine.read_image(image_path) {
            println!("failed to load image: {}", image_path);
            std::process::exit(1);
        }
    }
    
    // Main execution loop
    let mut running = true;
    while running {
        // Read instruction and increment PC
        let pc = virtual_machine.registers[Register::RPc as usize];
        let instr = virtual_machine.memory[pc as usize];
        virtual_machine.registers[Register::RPc as usize] += 1;

        // Get the top 4 bits for the opcode
        let operation = instr >> 12;
        
        // Convert the bits into our Operation enum
    match operation {
        x if x == Operation::ADD  as u16 => instructions::op_add(&mut virtual_machine, instr),
        x if x == Operation::AND  as u16 => instructions::op_and(&mut virtual_machine, instr),
        x if x == Operation::NOT  as u16 => instructions::op_not(&mut virtual_machine, instr),
        x if x == Operation::BR   as u16 => instructions::op_br(&mut virtual_machine, instr),
        x if x == Operation::JMP  as u16 => instructions::op_jmp(&mut virtual_machine, instr),
        x if x == Operation::JSR  as u16 => instructions::op_jsr(&mut virtual_machine, instr),
        x if x == Operation::LD   as u16 => instructions::op_ld(&mut virtual_machine, instr),
        x if x == Operation::LDI  as u16 => instructions::op_ldi(&mut virtual_machine, instr),
        x if x == Operation::LDR  as u16 => instructions::op_ldr(&mut virtual_machine, instr),
        x if x == Operation::LEA  as u16 => instructions::op_lea(&mut virtual_machine, instr),
        x if x == Operation::ST   as u16 => instructions::op_st(&mut virtual_machine, instr),
        x if x == Operation::STI  as u16 => instructions::op_sti(&mut virtual_machine, instr),
        x if x == Operation::STR  as u16 => instructions::op_str(&mut virtual_machine, instr),
        x if x == Operation::TRAP as u16 => instructions::op_trap(&mut virtual_machine, instr, &mut running),
    _ => {
        println!("operation doesnt exist: {:#X}", operation);
        running = false;
    }
}
    }
    
}
