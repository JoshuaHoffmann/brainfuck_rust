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
    pub fn new_from_raw(r: String) -> Interpreter {
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
                        Some(a) => program.push(Operator::CloseLoop(a)),
                        None    => panic!("Found ] without prior matching [ at pos {}\n", i),
                    }
                }
                _ => unreachable!("Illegal character found after filtering.")
            }
        }
        // Point open loops back to closing loops
        let mut i = program.len();
        let mut program_copy = program.clone();
        let mut loopstack_close:Vec<usize> = Vec::new();
        loop {
            i -= 1;
            match program_copy.pop().unwrap() {
                Operator::CloseLoop(_) => loopstack_close.push(i),
                Operator::OpenLoop(_) => {
                    match loopstack_close.pop() {
                        Some(a) => {
                            program[i] = Operator::OpenLoop(a);
                        },
                        None => panic!("Found [ without matching ] at pos {}\n", i),
                    }
                }
                _ => {},

            }
            if i == 0 {
                break;
            }
        }

        Interpreter {
            program: program,
            tape_array: Vec::new(),
            head_position: 0,
            program_counter: 0,
            halted: false,
        }
    }

    fn current_value(&self) -> u8 {
        *match self.tape_array.get(self.head_position) {
            None    => panic!("Access out of bounds error. The memory adress points to cell {}, but the memory is only {} large.", self.head_position, self.tape_array.len()),
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

    fn search_matching_closing(&self, pos_open: &usize) -> usize {
        let mut nesting_depth = 1;
        let mut pos_search = pos_open.clone();
        while nesting_depth != 0 {
            pos_search += 1;
            match self.program.get(pos_search) {
                None => panic!("Reached end of program without finding a matching ']' to the '[' at position {}. Nesting depth: {}", pos_open, nesting_depth),
                Some(op) => {
                    match op {
                        Operator::CloseLoop(_) => {nesting_depth -= 1}, // If there is another '[' increrase the nesting depth, because we stepped into a new loop.
                        Operator::OpenLoop(_)  => {nesting_depth += 1}, // If there is a ']' decrease the nesting depth, becuase we stepped out of a loop.
                        _ => (),
                    }
                }
            }
            // If the nesting depth is zero, we have found the matching ']' therfore we should break out of the loop and return the position.
            if nesting_depth == 0 {break}
        }
        return pos_search;
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