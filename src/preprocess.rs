use std::error::Error;

use crate::CompilerError;

pub fn check_loops(tokens: &[char]) -> Result<(), Vec<Box<dyn Error>>> {
    let mut st_stack = Vec::new();
    let mut end_stack = Vec::new();
    for (idx, tok) in tokens.iter().enumerate() {
        match *tok {
            '[' => st_stack.push(idx),
            ']' => end_stack.push(idx),
            _ => (),
        }
    }
    check_lp_errors(st_stack, end_stack)
}

pub fn check_lp_errors(
    st_stack: Vec<usize>,
    end_stack: Vec<usize>,
) -> Result<(), Vec<Box<dyn Error>>> {
    use std::cmp::Ordering::*;
    let (stack, symbol) = match st_stack.len().cmp(&end_stack.len()) {
        Greater => (st_stack, '['),
        Less => (end_stack, ']'),
        Equal => return Ok(()),
    };

    let mut errors = Vec::new();
    for idx in stack {
        errors.push(
            Box::new(CompilerError::Syntax(format!(
                "unmatched {} at index #{}",
                symbol, idx
            )))
            .into(),
        );
    }

    Err(errors)
}

pub fn strip_comments(prog: Vec<char>) -> Vec<char> {
    let mut processed_chars = Vec::new();
    let mut is_commented = false;
    let mut iter = prog.iter().peekable();

    while let Some(current_char) = iter.next() {
        match (current_char, iter.peek()) {
            ('/', Some(&'*')) if !is_commented => {
                iter.next();
                is_commented = true;
            }
            ('*', Some(&'/')) if is_commented => {
                iter.next();
                is_commented = false;
            }
            _ => {
                if !is_commented {
                    processed_chars.push(*current_char);
                }
            }
        }
    }

    processed_chars
}
