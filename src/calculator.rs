use std::collections::linked_list::Iter;
use std::collections::LinkedList;
use std::fmt;
use std::ops::Div;
use num_bigint::BigInt;
use core::ops::Rem;
use num_traits::pow::Pow;
use num_traits::identities::{Zero, One};
use num_traits::ToPrimitive;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpResult {
    Ok,
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    Number(BigInt),
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
            .push_back(StackValue::Number(BigInt::from(self.input_radix.clone())));
        Ok(OpResult::Ok)
    }

    pub fn get_output_radix(&mut self) -> Result<OpResult, String> {
        self.stack
            .push_back(StackValue::Number(BigInt::from(self.output_radix.clone())));
        Ok(OpResult::Ok)
    }

    pub fn get_precision(&mut self) -> Result<OpResult, String> {
        self.stack
            .push_back(StackValue::Number(BigInt::from(self.precision.clone())));
        Ok(OpResult::Ok)
    }

    pub fn set_input_radix(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .map(|value| {
                // TODO error handling
                self.input_radix = value.to_u8().unwrap();
            })
            .map(|_| OpResult::Ok)
    }

    pub fn set_output_radix(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .map(|value| {
                // TODO error handling
                self.output_radix = value.to_u8().unwrap();
            })
            .map(|_| OpResult::Ok)
    }

    pub fn set_precision(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .map(|value| {
                // TODO error handling
                self.precision = value.to_u8().unwrap();
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
                if y == BigInt::from(0) {
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
                if y == BigInt::from(0) {
                    Err("division by zero".to_owned())
                } else {
                    Ok((x, y))
                }
            })
            .and_then(|(x, y)| {
                let div = x.clone().div(&y);
                let rem = x.clone().rem(&y);
                self.push(StackValue::Number(rem))?;
                self.push(StackValue::Number(div))
            })
    }

    pub fn exp(&mut self) -> Result<OpResult, String> {
        // TODO error handling
        self.arg2_f64()
            .and_then(|(x, y)| self.push(StackValue::Number(x.pow(&y.to_biguint().unwrap()))))
    }

    pub fn sqrt(&mut self) -> Result<OpResult, String> {
        self.arg1_f64()
            .and_then(|x| self.push(StackValue::Number(x.sqrt())))
    }

    pub fn mod_exp(&mut self) -> Result<OpResult, String> {
        self.arg3_f64()
            .and_then(|(base, exponent, modulus)| {
                // TODO
                // if modulus <= BigInt::from(0) || modulus != modulus.trunc() {
                //     Err("base must be non-zero and an integer".to_owned())
                // } else if exponent < BigInt::from(0) {
                //     Err("exponent must be non-negative and an integer".to_owned())
                // } else if base != base.trunc() {
                //     Err("modulus must be an integer".to_owned())
                // } else {
                //     Ok((base, exponent, modulus))
                // }

                Ok((base, exponent, modulus))
            })
            .and_then(|(base, exponent, modulus)| {
                if modulus == BigInt::one() {
                    self.push(StackValue::Number(BigInt::zero()))
                } else {
                    let mut c = 1;
                    // TODO error handling
                    let base = base.to_u64().unwrap();
                    let exponent = exponent.to_u64().unwrap();
                    let modulus = modulus.to_u64().unwrap();
                    for _ in 0..exponent.to_u64().unwrap() {
                        c = (c * base) % modulus;
                    }
                    self.push(StackValue::Number(BigInt::from(c)))
                }
            })
    }

    fn arg1_f64(&mut self) -> Result<BigInt, String> {
        if self.stack.len() >= 1 {
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok(x)
        } else {
            Err("stack empty!".to_owned())
        }
    }

    fn arg2_f64(&mut self) -> Result<(BigInt, BigInt), String> {
        if self.stack.len() >= 2 {
            let StackValue::Number(y) = self.stack.pop_back().unwrap();
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok((x, y))
        } else {
            Err("stack empty!".to_owned())
        }
    }

    fn arg3_f64(&mut self) -> Result<(BigInt, BigInt, BigInt), String> {
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
    use num_bigint::BigInt;
    
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
        calculator.push(StackValue::Number(BigInt::from(42))).unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(42)));
    }

    #[test]
    fn test_pop() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(42)));
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(calculator.pop(), Some(StackValue::Number(BigInt::from(42))));
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
        calculator.stack.push_back(StackValue::Number(BigInt::from(42)));
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(calculator.peek(), Some(&StackValue::Number(BigInt::from(42))));
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
        calculator.stack.push_back(StackValue::Number(BigInt::from(1)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(2)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(3)));

        let mut result = Vec::new();
        for StackValue::Number(i) in calculator.iter() {
            result.push(i.clone());
        }

        assert_eq!(result, vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)]);
    }

    #[test]
    fn test_clear() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(1)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(2)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(3)));
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
        calculator.stack.push_back(StackValue::Number(BigInt::from(4)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(5)));
        calculator.reverse().unwrap();
        assert_eq!(calculator.stack.len(), 2);
        assert_eq!(*calculator.stack.front().unwrap(), StackValue::Number(BigInt::from(5)));
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(4)));
    }

    #[test]
    fn test_reverse_singleton() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(4)));
        calculator.reverse().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(4)));
    }

    #[test]
    fn test_add() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(4)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(5)));
        calculator.add().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(9)));
    }

    #[test]
    fn test_sub() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(4)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(5)));
        calculator.sub().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(-1)));
    }

    #[test]
    fn test_mul() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(4)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(5)));
        calculator.mul().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(20)));
    }

    #[test]
    fn test_div() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(20)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(5)));
        calculator.div().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(4)));
    }

    #[test]
    fn test_modulo() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(10)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(6)));
        calculator.modulo().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(4)));
    }

    #[test]
    fn test_div_rem() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(10)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(6)));
        calculator.div_rem().unwrap();
        assert_eq!(calculator.stack.len(), 2);
        assert_eq!(*calculator.stack.front().unwrap(), StackValue::Number(BigInt::from(4)));
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(1)));
    }

    #[test]
    fn test_exp() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(2)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(10)));
        calculator.exp().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(BigInt::from(1024))
        );
    }

    #[test]
    fn test_sqrt() {
        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(10_000)));
        calculator.sqrt().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(100)));
    }

    #[test]
    fn test_mod_exp() {
        // https://en.wikipedia.org/wiki/Modular_exponentiation

        let mut calculator = Calculator::new();
        calculator.stack.push_back(StackValue::Number(BigInt::from(4)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(13)));
        calculator.stack.push_back(StackValue::Number(BigInt::from(497)));
        calculator.mod_exp().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(*calculator.stack.back().unwrap(), StackValue::Number(BigInt::from(445)));
    }

    #[test]
    fn test_get_input_radix() {
        let mut calculator = Calculator::new();
        calculator.get_input_radix().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(BigInt::from(calculator.input_radix))
        );
    }

    #[test]
    fn test_get_output_radix() {
        let mut calculator = Calculator::new();
        calculator.get_output_radix().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(BigInt::from(calculator.output_radix))
        );
    }

    #[test]
    fn test_get_precision() {
        let mut calculator = Calculator::new();
        calculator.get_precision().unwrap();
        assert_eq!(calculator.stack.len(), 1);
        assert_eq!(
            *calculator.stack.back().unwrap(),
            StackValue::Number(BigInt::from(calculator.precision))
        );
    }

    #[test]
    fn test_set_input_radix() {
        let mut calculator = Calculator::new();
        calculator.push(StackValue::Number(BigInt::from(42))).unwrap();
        calculator.set_input_radix().unwrap();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.input_radix, 42);
        assert_eq!(calculator.output_radix, 10);
        assert_eq!(calculator.precision, 0);
    }

    #[test]
    fn test_set_output_radix() {
        let mut calculator = Calculator::new();
        calculator.push(StackValue::Number(BigInt::from(42))).unwrap();
        calculator.set_output_radix().unwrap();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.input_radix, 10);
        assert_eq!(calculator.output_radix, 42);
        assert_eq!(calculator.precision, 0);
    }

    #[test]
    fn test_set_precision() {
        let mut calculator = Calculator::new();
        calculator.push(StackValue::Number(BigInt::from(42))).unwrap();
        calculator.set_precision().unwrap();
        assert_eq!(calculator.stack.len(), 0);
        assert_eq!(calculator.input_radix, 10);
        assert_eq!(calculator.output_radix, 10);
        assert_eq!(calculator.precision, 42);
    }

    #[test]
    fn test_bignum() {
        let mut calculator = Calculator::new();
        let bignum = "123456789123456789".as_bytes();
        let expected = BigInt::parse_bytes("15241578780673678515622620750190521".as_bytes(), 10).unwrap();
        calculator.push(StackValue::Number(BigInt::parse_bytes(bignum, 10).unwrap())).unwrap();
        calculator.push(StackValue::Number(BigInt::from(2))).unwrap();
        calculator.exp().unwrap();
        assert_eq!(&StackValue::Number(expected), calculator.peek().unwrap());
    }
}
