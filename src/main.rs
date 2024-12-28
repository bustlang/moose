use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{} is not a valid option.", &args[1]);
        return;
    }

    println!("Moose - Package Manager for Bust");
    println!("================================");
    println!("moose install <package> - Install a Reef (package)");
    println!("moose remove <package> - Remove a Reef (package)");
    println!("moose run - Compile and run the project");
    println!("================================");
    println!("---> MOOSE IS A WORK IN PROGRESS PROJECT. NO FUNCTIONALITY IS IMPLEMENTED RIGHT NOW. <---");
}