use crate::interpreter::{str_to_program, Operator};

const TAPE_SIZE: usize = 1000;

fn push_code_indent(s: &mut String, p: &str, i: usize) {
    for x in 0..i {
        s.push_str("\t");
    }
    s.push_str(p);
}

pub fn compile_to_c(raw: String) -> Result<String, &'static str> {
    
    let program = str_to_program(raw)?;

    let mut c_code = String::new();
    let mut indent_depth = 0;

    // Open boilerplate
    push_code_indent(&mut c_code, "#include<stdio.h>\nint main() {\n", indent_depth);

    indent_depth += 1;

    // Initialize Tape, Tape pointer
    push_code_indent(&mut c_code, &format!("char tape [ {} ] = {{0}};\n", TAPE_SIZE), indent_depth);
    push_code_indent(&mut c_code,"char *ptr;\n", indent_depth);
    push_code_indent(&mut c_code, "ptr = &tape[0];\n", indent_depth);

    for (counter, &instruction) in program.iter().enumerate() {
        match instruction {
            Operator::IncrDataPtr => {
                push_code_indent(&mut c_code, "ptr++;\n", indent_depth)
            },
            Operator::DecrDataPtr => {
                push_code_indent(&mut c_code, "ptr--;\n", indent_depth)
            },
            Operator::IncrData => {
                push_code_indent(&mut c_code, "(*ptr)++;\n", indent_depth)
            },
            Operator::DecrData => {
                push_code_indent(&mut c_code, "(*ptr)--;\n", indent_depth)
            },
            Operator::Halt => {
                push_code_indent(&mut c_code, "return 0;\n", indent_depth)
            },
            Operator::InputData => {
                push_code_indent(&mut c_code, "*ptr = getchar();\n", indent_depth)
            },
            Operator::OutputData => {
                push_code_indent(&mut c_code, "putchar(*ptr);\n", indent_depth)
            },
            Operator::OpenLoop(_) => {
                push_code_indent(&mut c_code, "while (*ptr) {\n", indent_depth);
                indent_depth += 1;
            },
            Operator::CloseLoop(_) => {
                indent_depth -= 1;
                push_code_indent(&mut c_code, "}\n", indent_depth);
            }
            _ => return Err("Unknown character encounterd while generating C code."),
        }
    }
    


    // Close boilerplate
    push_code_indent(&mut c_code, "return 0;\n", indent_depth);
    indent_depth -= 1;

    push_code_indent(&mut c_code, "}", indent_depth);

    return Ok(c_code);
}