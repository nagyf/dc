extern crate clap;

pub mod calculator;
mod token;

use crate::calculator::{Calculator, OpResult, StackValue};
use crate::token::{tokenize, Op};
use std::io::Write;

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
            Err(err) => println!("{}", err),
        };
    }

    Ok(OpResult::Ok)
}

fn process_op(calculator: &mut Calculator, op: &Op) -> Result<OpResult, String> {
    match op {
        Op::GetInputRadix => calculator.get_input_radix(),
        Op::GetOutputRadix => calculator.get_output_radix(),
        Op::GetPrecision => calculator.get_precision(),
        Op::SetInputRadix => calculator.set_input_radix(),
        Op::SetOutputRadix => calculator.set_output_radix(),
        Op::SetPrecision => calculator.set_precision(),
        Op::Exit => Ok(OpResult::Exit),
        Op::PrintPeek => calculator
            .peek()
            .map(|value| {
                println!("{}", value);
                OpResult::Ok
            })
            .ok_or("stack empty!".to_owned()),
        Op::Clear => calculator.clear(),
        Op::PrintAll => {
            calculator.iter().for_each(|value| println!("{}", value));
            Ok(OpResult::Ok)
        }
        Op::PrintPop => calculator
            .pop()
            .map(|value| {
                print!("{}", value);
                std::io::stdout().flush().unwrap();
                OpResult::Ok
            })
            .ok_or("stack empty!".to_owned()),
        Op::Duplicate => {
            let value = calculator.peek().and_then(|value| match value {
                StackValue::Number(num) => Some(num),
            });

            if let Some(num) = value {
                let stack_value = StackValue::Number(*num);
                calculator.push(stack_value)
            } else {
                Ok(OpResult::Ok)
            }
        }
        Op::Reverse => calculator.reverse(),
        Op::Add => calculator.add(),
        Op::Sub => calculator.sub(),
        Op::Mul => calculator.mul(),
        Op::Div => calculator.div(),
        Op::Mod => calculator.modulo(),
        Op::DivRem => calculator.div_rem(),
        Op::Exp => calculator.exp(),
        Op::Sqrt => calculator.sqrt(),
        Op::ModExp => calculator.mod_exp(),
        Op::Push(num) => calculator.push(StackValue::Number(*num)),
    }
}

#[cfg(test)]
mod test {
    use crate::calculator::{Calculator, StackValue};
    use crate::process_input;

    #[test]
    fn test_execution_empty() {
        let mut calculator = Calculator::new();
        process_input(&mut calculator, "").unwrap();
        assert_eq!(calculator.peek(), None);
    }

    #[test]
    fn test_execution_only_whitespaces() {
        let mut calculator = Calculator::new();
        process_input(&mut calculator, "     \n   \t\r\n  ").unwrap();
        assert_eq!(calculator.peek(), None);
    }

    #[test]
    fn test_execution_basic() {
        let mut calculator = Calculator::new();
        // sqrt((((5 * 5 + 10) - 2) / 2) ^ 2)
        process_input(&mut calculator, "5d*5+10-2/2^v").unwrap();
        assert_eq!(*calculator.peek().unwrap(), StackValue::Number(10.0));
    }

    #[test]
    fn test_execution_basic_whitespaces() {
        let mut calculator = Calculator::new();
        // sqrt((((5 * 5 + 10) - 2) / 2) ^ 2)
        process_input(&mut calculator, "   5 d * 5 + 10 - 2 / 2 ^ v     ").unwrap();
        assert_eq!(*calculator.peek().unwrap(), StackValue::Number(10.0));
    }

    #[test]
    fn test_execution_duplicate() {
        let mut calculator = Calculator::new();
        process_input(&mut calculator, "5d*").unwrap();
        assert_eq!(*calculator.peek().unwrap(), StackValue::Number(25.0));
    }
}
