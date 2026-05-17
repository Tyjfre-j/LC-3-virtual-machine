use lc_3_virtual_machine::{
    hardware::{LC3, Register},
    isa::ConditionFlag,
    terminal::{disable_input_buffering, install_ctrlc_handler},
    utils::resolve_image_path,
    virtual_machine::run,
};

fn main() {
    // Disable input echo + line buffering so programs can poll the keyboard (MMIO).
    let _input_guard = disable_input_buffering().ok();
    if let Err(e) = install_ctrlc_handler() {
        eprintln!("warning: failed to install Ctrl+C handler: {e}");
    }

    // Handle CLI arguments (equivalent to argc/argv check)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!(
            "lc3 [image-file1] , [image-file2] ... this is how you load programs into the virtual machine"
        );
        std::process::exit(2);
    }
    // Initialize the LC-3 virtual machine
    let mut virtual_machine = LC3::new();
    virtual_machine.registers[Register::RCond as usize] = ConditionFlag::ZRO as u16;
    virtual_machine.registers[Register::RPc as usize] = 0x3000;

    // Load image files into VM memory
    for image_path in &args[1..] {
        let resolved = resolve_image_path(image_path);
        let resolved_str = resolved.to_string_lossy();
        if virtual_machine.read_image(&resolved_str).is_err() {
            println!("failed to load image: {}", resolved_str);
            std::process::exit(1);
        }
    }

    // Run the virtual machine
    run(&mut virtual_machine);
}
