mod interpreter;
mod compiler;

use crate::interpreter::Interpreter;
use std::env;
use std::fs;

fn print_usage() {
    println!("Usage: Brainfuck.exe <filename>");
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide only one file as argument.");
        print_usage();
        std::process::exit(0);
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the provided file.");

    let mut inter = Interpreter::new_from_raw(contents)?;
    inter.run_safe();

    return Ok(());
}
