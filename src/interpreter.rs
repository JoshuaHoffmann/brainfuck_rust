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
    OpenLoop,
    CloseLoop,
}

type Program = Vec<Operator>;

const STR_OPERATORS: &str = "<>+-,.[]";

pub struct Interpreter {
    program: Program,
    tape_array: [u8; 3000],
    head_position: usize,
    program_counter: usize,
    loop_stack: Vec<usize>,
}

impl Interpreter {
    pub fn new(p: Program) -> Interpreter {
        Interpreter {
            program: p,
            tape_array: [0; 3000],
            head_position: 0,
            program_counter: 0,
            loop_stack: Vec::new(),
        }
    }

    pub fn new_from_raw(r: String) -> Interpreter {
        Interpreter {
            program: r.chars()
            .filter( | & c| STR_OPERATORS.contains(c))
            .map( | c| match c {
            '>' => Operator::IncrDataPtr,
            '<' => Operator::DecrDataPtr,
            '+' => Operator::IncrData,
            '-' => Operator::DecrData,
            '.' => Operator::OutputData,
            ',' => Operator::InputData,
            '[' => Operator::OpenLoop,
            ']' => Operator::CloseLoop,
            _ => unreachable!("While trying to convert the characters to \
                                      the operator enum, a unknown character '{}' appeared", c)
            }).collect(),
            tape_array: [0; 3000],
            head_position: 0,
            program_counter: 0,
            loop_stack: Vec::new(),
    }
    }

    fn current_value(&self) -> u8 {self.tape_array[self.head_position]}

    fn current_value_as_char(&self) -> char {self.tape_array[self.head_position] as char}

    /// This function steps the Interpreter one step forward:
    /// * It executes the operation corresponding to the operator at the program counter.
    /// * It increases the program counter by one.
    fn step(&mut self) {
        let op:Operator = *self.program.get(self.program_counter).unwrap();
        if op == Operator::IncrDataPtr {
            self.head_position += 1;
        } else if op == Operator::DecrDataPtr {
            self.head_position -= 1;
        } else if op == Operator::IncrData {
            self.tape_array[self.head_position] += 1;
        } else if op == Operator::DecrData {
            self.tape_array[self.head_position] -= 1;
        } else if op == Operator::OpenLoop {
            if self.current_value() == 0 {
                self.program_counter = self.search_matching_closing(&self.program_counter);
            } else {
                self.loop_stack.push(self.program_counter);
            }
        } else if op == Operator::CloseLoop {
            if self.current_value() != 0 {
                self.program_counter = *self.loop_stack.last().unwrap();
            } else {
                self.loop_stack.pop();
            }
        } else if op == Operator::OutputData {
            print!("{}", self.current_value_as_char());
            stdout().flush();
        } else if op == Operator::InputData {
            self.tape_array[self.head_position] = read!();
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
                        Operator::CloseLoop => {nesting_depth -= 1}, // If there is another '[' increrase the nesting depth, because we stepped into a new loop.
                        Operator::OpenLoop  => {nesting_depth += 1}, // If there is a ']' decrease the nesting depth, becuase we stepped out of a loop.
                        _ => (),
                    }
                }
            }
            // If the nesting depth is zero, we have found the matching ']' therfore we should break out of the loop and return the position.
            if nesting_depth == 0 {break}
        }
        return pos_search;
    }

    /// This function just runs the interprreter forever until something breaks or panics manually.
    pub fn run_unsafe(&mut self) {loop {self.step()} }

    pub fn run_safe(&mut self) {
        loop {
            if self.program_counter > self.program.len() - 1 {
                println!("\nThe Program has ended.");
                break;
            }
            self.step();
        }

    }
}