mod interpreter;
mod compiler;

use compiler::compile_to_c;

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
        return Ok(());
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the provided file.");

    let c_code = compile_to_c(contents)?;
    print!("{}", c_code);

    return Ok(());
}
