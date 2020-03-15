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
        Op::GetInputRadix => {
            stack.get_input_radix();
            Ok(OpResult::Ok)
        },
        Op::GetOutputRadix => {
            stack.get_output_radix();
            Ok(OpResult::Ok)
        },
        Op::GetPrecision => {
            stack.get_precision();
            Ok(OpResult::Ok)
        },
        Op::SetInputRadix => {
            stack.set_input_radix().and(Ok(OpResult::Ok))
        },
        Op::SetOutputRadix => {
            stack.set_output_radix().and(Ok(OpResult::Ok))
        },
        Op::SetPrecision => {
            stack.set_precision().and(Ok(OpResult::Ok))
        },
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

#[cfg(test)]
mod test {
    use crate::stack::{Stack, StackValue};
    use crate::process_input;

    #[test]
    fn test_execution_basic() {
        let mut stack = Stack::new();
        // sqrt((((5 * 5 + 10) - 2) / 2) ^ 2)
        process_input(&mut stack, "5d*5+10-2/2^v");
        assert_eq!(*stack.peek().unwrap(), StackValue::Number(10.0));
    }

    #[test]
    fn test_execution_duplicate() {
        let mut stack = Stack::new();
        process_input(&mut stack, "5d*");
        assert_eq!(*stack.peek().unwrap(), StackValue::Number(25.0));
    }
}