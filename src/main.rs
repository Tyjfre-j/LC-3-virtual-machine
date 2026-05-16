use lc_3_virtual_machine::{
    hardware::{LC3, Register},
    isa::ConditionFlag,
    virtual_machine::run,
};
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
    
    // Run the virtual machine
    run(&mut virtual_machine);

}
