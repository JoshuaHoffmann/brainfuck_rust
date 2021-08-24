mod interpreter;

use crate::interpreter::Interpreter;
use std::env;
use std::fs;

fn print_usage() {
    println!("Usage: Brainfuck.exe <filename>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide only one filename.");
        print_usage();
        std::process::exit(0);
    }

    let filename = &args[2];

    let contents = fs::read_to_string(filename)
                      .expect("Something went wrong reading the file");

    let mut inter = Interpreter::new_from_raw(contents);
    inter.run_safe();
}
