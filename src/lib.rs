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
            Ok(OpResult::Exit) => return Ok(OpResult::Exit),
            Ok(_) => (),
            Err(err) => println!("{}", err)
        };
    }

    Ok(OpResult::Ok)
}

fn process_op(stack: &mut Stack, op: &Op) -> Result<OpResult, String> {
    match op {
        Op::Exit => {
            Ok(OpResult::Exit)
        }
        Op::PrintPeek => {
            stack.peek()
                .map(|value| {
                    println!("{}", value);
                    OpResult::Ok
                })
                .ok_or("stack empty!".to_owned())
        }
        Op::Clear => {
            stack.clear();
            Ok(OpResult::Ok)
        }
        Op::PrintAll => {
            stack.iter().for_each(|value| println!("{}", value));
            Ok(OpResult::Ok)
        }
        Op::PrintPop => {
            stack.pop().map(|value| {
                print!("{}", value);
                std::io::stdout().flush().unwrap();
                OpResult::Ok
            }).ok_or("stack empty!".to_owned())
        }
        Op::Duplicate => {
            let value = stack.peek().and_then(|value| {
                match value {
                    StackValue::Number(num) => Some(num)
                }
            });

            if let Some(num) = value {
                let stack_value = StackValue::Number(*num);
                stack.push(stack_value);
            }

            Ok(OpResult::Ok)
        }
        Op::Reverse => {
            stack.reverse();
            Ok(OpResult::Ok)
        }
        Op::Add => {
            stack.add().and(Ok(OpResult::Ok))
        }
        Op::Sub => {
            stack.sub().and(Ok(OpResult::Ok))
        }
        Op::Mul => {
            stack.mul().and(Ok(OpResult::Ok))
        }
        Op::Div => {
            stack.div().and(Ok(OpResult::Ok))
        }
        Op::Mod => {
            stack.modulo().and(Ok(OpResult::Ok))
        }
        Op::DivRem => {
            stack.div_rem().and(Ok(OpResult::Ok))
        }
        Op::Exp => {
            stack.exp().and(Ok(OpResult::Ok))
        }
        Op::Sqrt => {
            stack.sqrt().and(Ok(OpResult::Ok))
        }
        Op::ModExp => {
            stack.mod_exp().and(Ok(OpResult::Ok))
        }
        Op::Push(num) => {
            stack.push(StackValue::Number(*num));
            Ok(OpResult::Ok)
        }
    }
}