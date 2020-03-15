extern crate clap;

pub mod calculator;
mod token;

use std::io::Write;
use crate::calculator::{Calculator, StackValue};
use crate::token::{tokenize, Op};

pub enum OpResult {
    Ok,
    Exit
}

pub fn process_input(stack: &mut Calculator, str: &str) -> Result<OpResult, String> {
    let tokens = str
        .split_whitespace()
        .map(tokenize)
        .flatten()
        .collect::<Vec<Vec<Op>>>()
        .concat();

    for op in tokens {
        match process_op(stack, &op) {
            Ok(OpResult::Exit) => return Ok(OpResult::Exit),
            Ok(_) => (),
            Err(err) => println!("{}", err)
        };
    }

    Ok(OpResult::Ok)
}

fn process_op(calculator: &mut Calculator, op: &Op) -> Result<OpResult, String> {
    match op {
        Op::GetInputRadix => {
            calculator.get_input_radix();
            Ok(OpResult::Ok)
        },
        Op::GetOutputRadix => {
            calculator.get_output_radix();
            Ok(OpResult::Ok)
        },
        Op::GetPrecision => {
            calculator.get_precision();
            Ok(OpResult::Ok)
        },
        Op::SetInputRadix => {
            calculator.set_input_radix().and(Ok(OpResult::Ok))
        },
        Op::SetOutputRadix => {
            calculator.set_output_radix().and(Ok(OpResult::Ok))
        },
        Op::SetPrecision => {
            calculator.set_precision().and(Ok(OpResult::Ok))
        },
        Op::Exit => {
            Ok(OpResult::Exit)
        }
        Op::PrintPeek => {
            calculator.peek()
                .map(|value| {
                    println!("{}", value);
                    OpResult::Ok
                })
                .ok_or("stack empty!".to_owned())
        }
        Op::Clear => {
            calculator.clear();
            Ok(OpResult::Ok)
        }
        Op::PrintAll => {
            calculator.iter().for_each(|value| println!("{}", value));
            Ok(OpResult::Ok)
        }
        Op::PrintPop => {
            calculator.pop().map(|value| {
                print!("{}", value);
                std::io::stdout().flush().unwrap();
                OpResult::Ok
            }).ok_or("stack empty!".to_owned())
        }
        Op::Duplicate => {
            let value = calculator.peek().and_then(|value| {
                match value {
                    StackValue::Number(num) => Some(num)
                }
            });

            if let Some(num) = value {
                let stack_value = StackValue::Number(*num);
                calculator.push(stack_value);
            }

            Ok(OpResult::Ok)
        }
        Op::Reverse => {
            calculator.reverse();
            Ok(OpResult::Ok)
        }
        Op::Add => {
            calculator.add().and(Ok(OpResult::Ok))
        }
        Op::Sub => {
            calculator.sub().and(Ok(OpResult::Ok))
        }
        Op::Mul => {
            calculator.mul().and(Ok(OpResult::Ok))
        }
        Op::Div => {
            calculator.div().and(Ok(OpResult::Ok))
        }
        Op::Mod => {
            calculator.modulo().and(Ok(OpResult::Ok))
        }
        Op::DivRem => {
            calculator.div_rem().and(Ok(OpResult::Ok))
        }
        Op::Exp => {
            calculator.exp().and(Ok(OpResult::Ok))
        }
        Op::Sqrt => {
            calculator.sqrt().and(Ok(OpResult::Ok))
        }
        Op::ModExp => {
            calculator.mod_exp().and(Ok(OpResult::Ok))
        }
        Op::Push(num) => {
            calculator.push(StackValue::Number(*num));
            Ok(OpResult::Ok)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::calculator::{Calculator, StackValue};
    use crate::process_input;

    #[test]
    fn test_execution_basic() {
        let mut calculator = Calculator::new();
        // sqrt((((5 * 5 + 10) - 2) / 2) ^ 2)
        process_input(&mut calculator, "5d*5+10-2/2^v");
        assert_eq!(*calculator.peek().unwrap(), StackValue::Number(10.0));
    }

    #[test]
    fn test_execution_duplicate() {
        let mut calculator = Calculator::new();
        process_input(&mut calculator, "5d*");
        assert_eq!(*calculator.peek().unwrap(), StackValue::Number(25.0));
    }

    #[test]
    fn test_execution_whitespaces() {
        let mut calculator = Calculator::new();
        process_input(&mut calculator, "10 5 2 * /");
        assert_eq!(*calculator.peek().unwrap(), StackValue::Number(1.0));
    }
}