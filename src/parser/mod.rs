mod file;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use iter_tools::Itertools;

pub fn parse_file(file: &mut BufReader<File>, name: String) -> Result<file::File, String> {
    let mut definitions = Vec::new();
    for line in file.lines() {
        let unwrapped = line.unwrap();
        let trimmed = unwrapped.trim();
        if trimmed.len() == 0 {
            continue;
        }
        definitions.push(parse_definition(trimmed, &definitions)?);
    }
    Ok(file::File { name, definitions })
}

pub fn parse_definition(
    line: &str,
    prev_defs: &[file::Definition],
) -> Result<file::Definition, String> {
    // check line length
    if line.len() < 3 {
        return Err("Line too short".to_string());
    }

    // check let token presence
    if line[0..=2] != *"let" {
        return Err("Line does not contain let token".to_string());
    }
    match line.chars().nth(3) {
        None => return Err("Line too short".to_string()),
        Some(n) => {
            if !n.is_whitespace() {
                return Err("Line does not contain let token".to_string());
            }
        }
    }

    // take in definition name
    let mut end = 4;
    while end < line.len() && !line.chars().nth(end).unwrap().is_whitespace() {
        end += 1;
    }
    let name = line[4..end].to_string();

    // check set token presence
    if line.chars().nth(end + 1).is_none() {
        return Err("Line too short".to_string());
    }
    if line[end + 1..=end + 2] != *"=>" {
        return Err("Line does not contain equality token".to_string());
    }
    if line.chars().nth(end + 4).is_none() {
        return Err("Line too short".to_string());
    }

    // parse body
    let value = parse_body(&line[end + 4..], prev_defs)?;

    // return constructed definition
    Ok(file::Definition { name, value })
}

pub fn parse_body(line: &str, prev_defs: &[file::Definition]) -> Result<file::Term, String> {
    let mut start: usize;
    let mut current: usize = 0;
    let mut env_variables: Vec<Vec<String>> = Vec::new();
    let mut dump: Vec<(Vec<file::Term>, i32, i32)> = Vec::new();
    let mut nesting_level = 0;
    let mut stack: Vec<file::Term> = Vec::new();
    let mut op_stack_size = 0;
    let mut first_op = true;
    let chars: Vec<char> = line.chars().collect();
    while current < line.len() {
        match chars[current] {
            '\\' => {
                if chars[current + 1] != '(' {
                    return Err("Lambda slash not followed by parenthesis".to_string());
                }
                current += 2;
                start = current + 2;
                let mut args: Vec<String> = Vec::new();
                let mut id_started = false;
                while chars[current] != ')' {
                    if chars[current].is_whitespace() {
                        current += 1;
                        continue;
                    }
                    if chars[current] == ';' {
                        if !id_started {
                            return Err("Empty argument name".to_string());
                        }
                        args.push(line[start..current].to_string());
                        id_started = false;
                        current += 1;
                        continue;
                    }
                    if !id_started {
                        start = current;
                        id_started = true;
                        current += 1;
                        continue;
                    }
                    current += 1;
                    continue;
                }
                env_variables.push(args);
                dump.push((stack, nesting_level, op_stack_size));
                stack = Vec::new();
                nesting_level = 0;
                op_stack_size = 0;
                first_op = true;
            }
            '(' => {
                if !first_op && op_stack_size == nesting_level {
                    op_stack_size += 1;
                } else if !first_op {
                    let argument = Box::new(stack.pop().unwrap());
                    let function = Box::new(stack.pop().unwrap());
                    stack.push(file::Term::App(file::Application { argument, function }));
                }
                first_op = true;
                nesting_level += 1;
            }
            ')' => {
                if nesting_level == 0 {
                    if op_stack_size == 1 {
                        let argument = Box::new(stack.pop().unwrap());
                        let function = Box::new(stack.pop().unwrap());
                        stack.push(file::Term::App(file::Application { argument, function }));
                    }
                    if stack.len() != 1 {
                        return Err("Impossible".to_string());
                    }
                    let complete = file::Term::Lam(file::Lambda {
                        arg_count: env_variables.last().unwrap().len(),
                        body: Box::new(stack[0].clone()),
                    });
                    (stack, nesting_level, op_stack_size) = dump.pop().unwrap();
                    env_variables.pop();
                    stack.push(complete);
                }
                nesting_level -= 1;
                if op_stack_size > nesting_level {
                    let argument = Box::new(stack.pop().unwrap());
                    let function = Box::new(stack.pop().unwrap());
                    stack.push(file::Term::App(file::Application { argument, function }));

                    op_stack_size -= 1;
                }
            }
            c => {
                if c.is_whitespace() {
                    current += 1;
                    continue;
                }

                if !c.is_alphanumeric() {
                    // Is not an identifier
                    return Err("TODO".to_string());
                }

                start = current;
                while current < chars.len() && chars[current].is_alphanumeric() {
                    current += 1;
                }
                let identifier = line[start..current].to_string();
                current -= 1; // the code will bump current back up later

                // Convert to DeBruijin variable (or insert referenced def)
                let resolved: file::Term;
                match env_variables
                    .iter()
                    .flatten()
                    .rev()
                    .find_position(|x| identifier.eq(*x))
                {
                    None => {
                        // Maybe it's a previous def
                        match prev_defs.iter().find(|x| identifier.eq(&x.name)) {
                            None => return Err(format!("Invalid identifier {}", identifier)),
                            Some(d) => resolved = d.value.clone(),
                        }
                    }
                    Some(p) => resolved = file::Term::Var(file::Variable { index: p.0 as u32 }),
                }

                if !first_op && op_stack_size == nesting_level {
                    op_stack_size += 1;
                } else if !first_op {
                    let argument = Box::new(stack.pop().unwrap());
                    let function = Box::new(stack.pop().unwrap());
                    stack.push(file::Term::App(file::Application { argument, function }));
                }

                stack.push(resolved);
                first_op = false;
            }
        }
        current += 1;
    }

    if op_stack_size == 1 {
        let argument = Box::new(stack.pop().unwrap());
        let function = Box::new(stack.pop().unwrap());
        stack.push(file::Term::App(file::Application { argument, function }));
    } else if op_stack_size > 1 {
        return Err("Mismatched parenthesis".to_string());
    }

    return Ok(stack[0].clone());
}
