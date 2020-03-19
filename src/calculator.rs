use std::collections::linked_list::Iter;
use std::collections::LinkedList;
use std::fmt;
use std::ops::Div;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpResult {
    Ok,
    Exit,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StackValue {
    Number(f64),
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackValue::Number(x) => write!(f, "{}", x),
        }
    }
}

pub struct Calculator {
    stack: LinkedList<StackValue>,
    input_radix: u8,
    output_radix: u8,
    precision: u8,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            stack: LinkedList::new(),
            input_radix: 10,
            output_radix: 10,
            precision: 0,
        }
    }

    pub fn get_input_radix(&mut self) -> Result<OpResult, String> {
        self.stack
            .push_back(StackValue::Number(self.input_radix as f64));
        Ok(OpResult::Ok)
    }

    pub fn get_output_radix(&mut self) -> Result<OpResult, String> {
        self.stack
            .push_back(StackValue::Number(self.output_radix as f64));
        Ok(OpResult::Ok)
    }

    pub fn get_precision(&mut self) -> Result<OpResult, String> {
        self.stack
            .push_back(StackValue::Number(self.precision as f64));
        Ok(OpResult::Ok)
    }

    pub fn set_input_radix(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .map(|value| {
                self.input_radix = value as u8;
            })
            .map(|_| OpResult::Ok)
    }

    pub fn set_output_radix(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .map(|value| {
                self.output_radix = value as u8;
            })
            .map(|_| OpResult::Ok)
    }

    pub fn set_precision(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .map(|value| {
                self.precision = value as u8;
            })
            .map(|_| OpResult::Ok)
    }

    pub fn iter(&self) -> Iter<StackValue> {
        self.stack.iter()
    }

    pub fn push(&mut self, value: StackValue) -> Result<OpResult, String> {
        self.stack.push_back(value);
        Ok(OpResult::Ok)
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop_back()
    }

    pub fn peek(&self) -> Option<&StackValue> {
        self.stack.back()
    }

    pub fn clear(&mut self) -> Result<OpResult, String> {
        self.stack.clear();
        Ok(OpResult::Ok)
    }

    pub fn reverse(&mut self) -> Result<OpResult, String> {
        if self.stack.len() >= 2 {
            let first = self.stack.pop_back().unwrap();
            let second = self.stack.pop_back().unwrap();

            self.stack.push_back(first);
            self.stack.push_back(second);
        }

        Ok(OpResult::Ok)
    }

    pub fn add(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| self.push(StackValue::Number(x + y)))
    }

    pub fn sub(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| self.push(StackValue::Number(x - y)))
    }

    pub fn mul(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| self.push(StackValue::Number(x * y)))
    }

    pub fn div(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| {
                if y == 0f64 {
                    Err("division by zero".to_owned())
                } else {
                    Ok((x, y))
                }
            })
            .and_then(|(x, y)| self.push(StackValue::Number(x / y)))
    }

    pub fn modulo(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| self.push(StackValue::Number(x % y)))
    }

    pub fn div_rem(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| {
                if y == 0f64 {
                    Err("division by zero".to_owned())
                } else {
                    Ok((x, y))
                }
            })
            .and_then(|(x, y)| {
                let div = x.div(y).floor();
                let rem = x % y;
                self.push(StackValue::Number(rem))?;
                self.push(StackValue::Number(div))
            })
    }

    pub fn exp(&mut self) -> Result<OpResult, String> {
        self.arg2_f64()
            .and_then(|(x, y)| self.push(StackValue::Number(x.powf(y))))
    }

    pub fn sqrt(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .and_then(|x| self.push(StackValue::Number(x.sqrt())))
    }

    pub fn mod_exp(&mut self) -> Result<OpResult, String> {
        self.arg3_f64()
            .and_then(|(base, exponent, modulus)| {
                if modulus <= 0.0 || modulus != modulus.trunc() {
                    Err("base must be non-zero and an integer".to_owned())
                } else if exponent < 0.0 {
                    Err("exponent must be non-negative and an integer".to_owned())
                } else if base != base.trunc() {
                    Err("modulus must be an integer".to_owned())
                } else {
                    Ok((base as i64, exponent as i64, modulus as i64))
                }
            })
            .and_then(|(base, exponent, modulus)| {
                if modulus == 1 {
                    self.push(StackValue::Number(0f64))
                } else {
                    let mut c = 1;
                    for _ in 0..exponent {
                        c = (c * base) % modulus;
                    }
                    self.push(StackValue::Number(c as f64))
                }
            })
    }

    fn arg1_f64(&mut self) -> Result<f64, String> {
        if self.stack.len() >= 1 {
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok(x)
        } else {
            Err("stack empty!".to_owned())
        }
    }

    fn arg2_f64(&mut self) -> Result<(f64, f64), String> {
        if self.stack.len() >= 2 {
            let StackValue::Number(y) = self.stack.pop_back().unwrap();
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok((x, y))
        } else {
            Err("stack empty!".to_owned())
        }
    }

    fn arg3_f64(&mut self) -> Result<(f64, f64, f64), String> {
        if self.stack.len() >= 3 {
            let StackValue::Number(z) = self.stack.pop_back().unwrap();
            let StackValue::Number(y) = self.stack.pop_back().unwrap();
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok((x, y, z))
        } else {
            Err("stack empty!".to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::calculator::*;

    #[test]
    fn test_new() {
        let calculator = Calculator::new();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.precision, 0);
        assert_eq!(calculator.input_radix, 10);
        assert_eq!(calculator.output_radix, 10);
    }

    #[test]
    fn test_push() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.stack.len(), 0);
        calculator.push(StackValue::Number(42.0)).unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(42.0));
    }

    #[test]
    fn test_pop() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(42.0));
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(calculator.pop(), Some(StackValue::Number(42.0)));
        assert_eq!(calculator.stack.len(), 0);
    }

    #[test]
    fn test_pop_empty() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.pop(), None);
        assert_eq!(calculator.stack.len(), 0);
    }

    #[test]
    fn test_peek() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(42.0));
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(calculator.peek(), Some(&StackValue::Number(42.0)));
        assert_eq!(calculator.stack.len(), 1);
    }

    #[test]
    fn test_peek_empty() {
        let calculator = Calculator::new();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.peek(), None);
        assert_eq!(calculator.stack.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(1.0));
        calculator.stack.push_back(StackValue::Number(2.0));
        calculator.stack.push_back(StackValue::Number(3.0));

        let mut result = Vec::new();
        for StackValue::Number(i) in calculator.iter() {
            result.push(*i);
        }

        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_clear() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(1.0));
        calculator.stack.push_back(StackValue::Number(2.0));
        calculator.stack.push_back(StackValue::Number(3.0));
        assert_eq!(calculator.stack.len(), 3);
        calculator.clear().unwrap();
        assert_eq!(calculator.stack.len(), 0);
    }

    #[test]
    fn test_clear_empty() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.stack.len(), 0);
        calculator.clear().unwrap();
        assert_eq!(calculator.stack.len(), 0);
    }

    #[test]
    fn test_reverse() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(4.0));
        calculator.stack.push_back(StackValue::Number(5.0));
        calculator.reverse().unwrap();
        assert_eq!(calculator.stack.len(), 2);
        assert_eq!(*calculator.stack.front().unwrap(), StackValue::Number(5.0));
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_reverse_singleton() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(4.0));
        calculator.reverse().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_add() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(4.0));
        calculator.stack.push_back(StackValue::Number(5.0));
        calculator.add().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(9.0));
    }

    #[test]
    fn test_sub() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(4.0));
        calculator.stack.push_back(StackValue::Number(5.0));
        calculator.sub().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(-1.0));
    }

    #[test]
    fn test_mul() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(4.0));
        calculator.stack.push_back(StackValue::Number(5.0));
        calculator.mul().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(20.0));
    }

    #[test]
    fn test_div() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(20.0));
        calculator.stack.push_back(StackValue::Number(5.0));
        calculator.div().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_modulo() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(10.0));
        calculator.stack.push_back(StackValue::Number(6.0));
        calculator.modulo().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_div_rem() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(10.0));
        calculator.stack.push_back(StackValue::Number(6.0));
        calculator.div_rem().unwrap();
        assert_eq!(calculator.stack.len(), 2);
        assert_eq!(*calculator.stack.front().unwrap(), StackValue::Number(4.0));
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(1.0));
    }

    #[test]
    fn test_exp() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(2.0));
        calculator.stack.push_back(StackValue::Number(10.0));
        calculator.exp().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(1024.0)
        );
    }

    #[test]
    fn test_sqrt() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(10_000.0));
        calculator.sqrt().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(100.0));
    }

    #[test]
    fn test_mod_exp() {
        // https://en.wikipedia.org/wiki/Modular_exponentiation

        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(4.0));
        calculator.stack.push_back(StackValue::Number(13.0));
        calculator.stack.push_back(StackValue::Number(497.0));
        calculator.mod_exp().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(445.0));
    }

    #[test]
    fn test_get_input_radix() {
        let mut calculator = Calculator::new();
        calculator.get_input_radix().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(calculator.input_radix as f64)
        );
    }

    #[test]
    fn test_get_output_radix() {
        let mut calculator = Calculator::new();
        calculator.get_output_radix().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(calculator.output_radix as f64)
        );
    }

    #[test]
    fn test_get_precision() {
        let mut calculator = Calculator::new();
        calculator.get_precision().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(calculator.precision as f64)
        );
    }

    #[test]
    fn test_set_input_radix() {
        let mut calculator = Calculator::new();
        calculator.push(StackValue::Number(42f64)).unwrap();
        calculator.set_input_radix().unwrap();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.input_radix, 42);
        assert_eq!(calculator.output_radix, 10);
        assert_eq!(calculator.precision, 0);
    }

    #[test]
    fn test_set_output_radix() {
        let mut calculator = Calculator::new();
        calculator.push(StackValue::Number(42f64)).unwrap();
        calculator.set_output_radix().unwrap();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.input_radix, 10);
        assert_eq!(calculator.output_radix, 42);
        assert_eq!(calculator.precision, 0);
    }

    #[test]
    fn test_set_precision() {
        let mut calculator = Calculator::new();
        calculator.push(StackValue::Number(42f64)).unwrap();
        calculator.set_precision().unwrap();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.input_radix, 10);
        assert_eq!(calculator.output_radix, 10);
        assert_eq!(calculator.precision, 42);
    }
}
