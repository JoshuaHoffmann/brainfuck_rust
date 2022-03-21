use std::io::{stdout, Write};
use text_io::read;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Operator {
    IncrDataPtr,
    DecrDataPtr,
    IncrData,
    DecrData,
    OutputData,
    InputData,
    OpenLoop(usize),
    CloseLoop(usize),
    Halt,
}

type Program = Vec<Operator>;
type Cell = u8;

const STR_OPERATORS: &str = "<>+-,.[]~";

pub struct Interpreter {
    program: Program,
    tape_array: Vec<Cell>,
    head_position: usize,
    program_counter: usize,
    halted: bool,
}

impl Interpreter {
    pub fn new(p: Program) -> Interpreter {
        Interpreter {
            program: p,
            tape_array: Vec::new(),
            head_position: 0,
            program_counter: 0,
            halted: false,
        }
    }

    /// Create a [Interpreter] struct from a raw string, by filtering for allowed [STR_OPERATORS] and then parsing them to a Vec of [Operator].
    pub fn new_from_raw(r: String) -> Result<Interpreter, String> {
        let program:Program = str_to_program(r)?;

        Ok (Interpreter {
            program: program,
            tape_array: Vec::new(),
            head_position: 0,
            program_counter: 0,
            halted: false,
        })
    }

    fn current_value(&self) -> u8 {
        *match self.tape_array.get(self.head_position) {
            None    => panic!("Access out of bounds error. The memory adress points to cell {}, but the memory tape is only {} large.", self.head_position, self.tape_array.len()),
            Some(a) => a,
        }
    }

    fn current_value_as_char(&self) -> char {self.current_value() as char}

    /// This function steps the Interpreter one step forward:
    /// * It executes the operation corresponding to the operator at the program counter.
    /// * It increases the program counter by one.
    fn step(&mut self) {
        // Try to get the operator at the program counter, if there is no way to get it safely, panic.
        let op = *match self.program.get(self.program_counter) {
            None    => panic!("Program counter: {} exceeded program length: {}.", self.program_counter, self.program.len()),
            Some(a) => a,
        };
        
        match op {
            Operator::IncrDataPtr => {
                self.head_position += 1;
                if self.head_position >= self.tape_array.len() {
                    self.tape_array.push(0);
                }
            },
            Operator::DecrDataPtr => {
                self.head_position -= 1;
            },
            Operator::DecrData => {
                self.tape_array[self.head_position] += 1;
            },
            Operator::IncrData => {
                self.tape_array[self.head_position] -= 1;
            },
            Operator::OpenLoop(a) => {
                if self.current_value() == 0 {
                    self.program_counter = a;
                }
            },
            Operator::CloseLoop(a) => {
                if self.current_value() != 0 {
                    self.program_counter = a;
                }
            },
            Operator::OutputData => {
                print!("{}", self.current_value_as_char());
                stdout().flush().expect("Something went wrong while trying to flush the stdout buffer");
            },
            Operator::InputData => {
                let c:char = read!();
                self.tape_array[self.head_position] = c as u8;
            },
            Operator::Halt => {
                self.halted = true;
                return;
            }
        }
        self.program_counter += 1;
    }

    /// This function just calls the step function until there is a halt operator or a panic.
    pub fn run_unsafe(&mut self) {
        while !self.halted {
            self.step()
        }
    }

    /// This function steps through the program until it reaches the end of the programm or the halt operator.
    pub fn run_safe(&mut self) {
        loop {
            if self.program_counter > self.program.len() - 1 || self.halted {
                println!("\nThe Program has ended.");
                break;
            }
            self.step();
        }
    }
    
}

pub fn str_to_program(r: String) -> Result<Program, String> {
    let filterd:Vec<char> = r.chars().filter( |&c| STR_OPERATORS.contains(c)).collect();
    let mut program:Vec<Operator> = Vec::new();
    let mut loopstack_open:Vec<usize> = Vec::new();
    for (i,c) in filterd.iter().enumerate() {
        match c {
            '<' => program.push(Operator::DecrDataPtr),
            '>' => program.push(Operator::IncrDataPtr),
            '+' => program.push(Operator::IncrData),
            '-' => program.push(Operator::DecrData),
            '.' => program.push(Operator::OutputData),
            ',' => program.push(Operator::InputData),
            '~' => program.push(Operator::Halt),
            '[' => {
                loopstack_open.push(i);
                program.push(Operator::OpenLoop(0));
            },
            ']' => {
                match loopstack_open.pop() {
                    Some(a) => {
                        program.push(Operator::CloseLoop(a));
                        program[a] = Operator::OpenLoop(i);
                    },
                    None    => {
                        return Err(format!("Found ] without prior matching [ at pos {}\n", i))
                    },
                }
            }
            _ => {
                return Err(format!("Illegal character '{}' found after filtering.\n", c))
            }
        }
    }
    if !loopstack_open.is_empty() {
        return Err(format!("Not all [ closed with ]. Loopstack: {:?}\n", loopstack_open));
    }
    return Ok(program);
}