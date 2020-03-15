pub mod stack;
mod token;

use std::io::Write;
use crate::stack::{Stack, StackValue};
use crate::token::{tokenize, Op};

pub enum OpResult {
    Ok,
    Exit
}

pub fn process_input(stack: &mut Stack, str: &str) -> Result<OpResult, String> {
    let tokens = tokenize(str)?;
    for op in tokens {
        match process_op(stack, &op) {
            OpResult::Exit => return Ok(OpResult::Exit),
            _ => ()
        };
    }

    Ok(OpResult::Ok)
}

fn process_op(stack: &mut Stack, op: &Op) -> OpResult {
    match op {
        Op::Exit => {
            OpResult::Exit
        }
        Op::PrintPeek => {
            stack.peek().map(|value| {
                println!("{}", value);
            }).or_else(|| {
                println!("stack empty!");
                None
            });
            OpResult::Ok
        }
        Op::Clear => {
            stack.clear();
            OpResult::Ok
        }
        Op::PrintAll => {
            stack.iter().for_each(|value| println!("{}", value));
            OpResult::Ok
        }
        Op::PrintPop => {
            stack.pop().map(|value| {
                print!("{}", value);
                std::io::stdout().flush();
            }).or_else(|| {
                println!("stack empty");
                None
            });
            OpResult::Ok
        }
        Op::Duplicate => {
            let value = stack.peek().and_then(|value| {
                match value {
                    StackValue::Number(num) => Some(num),
                    _ => None
                }
            });

            if let Some(num) = value {
                stack.push(StackValue::Number(*num));
            }

            OpResult::Ok
        }
        Op::Add => {
            stack.add().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::Sub => {
            stack.sub().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::Mul => {
            stack.mul().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::Div => {
            stack.div().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::Mod => {
            stack.modulo().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::DivRem => {
            stack.div_rem().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::Exp => {
            stack.exp().map_err(|error| {
                println!("{}", error);
            });
            OpResult::Ok
        }
        Op::Push(num) => {
            stack.push(StackValue::Number(*num));
            OpResult::Ok
        }
    }
}