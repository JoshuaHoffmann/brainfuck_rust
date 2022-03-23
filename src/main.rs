mod interpreter;
mod compiler;
mod utils;

use compiler::compile_to_c;

use crate::interpreter::Interpreter;
use crate::utils::new_extension;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn print_usage() {
    println!("Usage: Brainfuck.exe <com|int|deb> <filename>");
}

fn main() -> Result<(), String> {
    // TODO: Better argparse, maybe clap?
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Please provide exactly 2 arguments.");
        print_usage();
        return Ok(());
    }

    let path = &args[2];

    // TODO: Don't panic
    let contents = fs::read_to_string(path).expect("Something went wrong reading the provided file");

    let mode = &args[1];
    if mode == "com" {
        let c_code = compile_to_c(contents)?;
        let new_file = new_extension(path, ".c")?;
        // TODO: Don't panic, better error handling for consistency.
        // TODO: Save compiled file to path at compile src.
        let mut f = File::create(new_file).expect("Couldn't write compiled file");
        f.write_all(c_code.as_bytes()).expect("Couldn't write compiled code to allready opened file");
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

