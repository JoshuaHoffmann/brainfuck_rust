mod interpreter;
mod compiler;

use compiler::compile_to_c;

use crate::interpreter::Interpreter;
use std::env;
use std::fs;

fn print_usage() {
    println!("Usage: Brainfuck.exe <com|int|deb> <filename>");
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Please provide exactly 2 arguments.");
        print_usage();
        return Ok(());
    }

    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the provided file.");

    let mode = &args[1];
    if mode == "com" {
        let c_code = compile_to_c(contents)?;
        print!("{}", c_code);
    } else if mode == "int" {
        let mut inter = Interpreter::new_from_raw(contents)?;
        inter.run_safe()?;
    } else if mode == "deb" {
        let mut inter = Interpreter::new_from_raw(contents)?;
        inter.run_debug()?;
    } else {
        print_usage();
    }

    return Ok(());
}
