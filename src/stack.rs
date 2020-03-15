use std::collections::LinkedList;
use std::fmt;
use std::collections::linked_list::Iter;
use std::ops::Div;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StackValue {
    Number(f64)
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackValue::Number(x) => write!(f, "{}", x)
        }
    }
}

pub struct Stack {
    stack: LinkedList<StackValue>,
    input_radix: u8,
    output_radix: u8,
    precision: u8
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: LinkedList::new(),
            input_radix: 10,
            output_radix: 10,
            precision: 0
        }
    }

    pub fn get_input_radix(&mut self) {
        self.stack.push_back(StackValue::Number(self.input_radix as f64));
    }

    pub fn get_output_radix(&mut self) {
        self.stack.push_back(StackValue::Number(self.output_radix as f64));
    }

    pub fn get_precision(&mut self) {
        self.stack.push_back(StackValue::Number(self.precision as f64));
    }

    pub fn set_input_radix(&mut self) -> Result<(), String> {
        self.arg1_f64().map(|value| {
            self.input_radix = value as u8;
        })
    }

    pub fn set_output_radix(&mut self) -> Result<(), String> {
        self.arg1_f64().map(|value| {
            self.output_radix = value as u8;
        })
    }

    pub fn set_precision(&mut self) -> Result<(), String> {
        self.arg1_f64().map(|value| {
            self.precision = value as u8;
        })
    }

    pub fn iter(&self) -> Iter<StackValue> {
        self.stack.iter()
    }

    pub fn push(&mut self, value: StackValue) {
        self.stack.push_back(value);
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop_back()
    }

    pub fn peek(&self) -> Option<&StackValue> {
        self.stack.back()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn reverse(&mut self) {
        if self.stack.len() >= 2 {
            let first = self.stack.pop_back().unwrap();
            let second = self.stack.pop_back().unwrap();

            self.stack.push_back(first);
            self.stack.push_back(second);
        }
    }

    pub fn add(&mut self) -> Result<(), String> {
        self.arg2_f64().map(|(x,y)| {
            self.push(StackValue::Number(x + y));
        })
    }

    pub fn sub(&mut self) -> Result<(), String> {
        self.arg2_f64().map(|(x,y)| {
            self.push(StackValue::Number(x - y));
        })
    }

    pub fn mul(&mut self) -> Result<(), String> {
        self.arg2_f64().map(|(x,y)| {
            self.push(StackValue::Number(x * y));
        })
    }

    pub fn div(&mut self) -> Result<(), String> {
        self.arg2_f64()
            .and_then(|(x, y)| {
                if y == 0f64 { Err("division by zero".to_owned()) } else { Ok((x, y)) }
            })
            .map(|(x,y)| {
                self.push(StackValue::Number(x / y));
            })
    }

    pub fn modulo(&mut self) -> Result<(), String> {
        self.arg2_f64().map(|(x,y)| {
            self.push(StackValue::Number(x % y));
        })
    }

    pub fn div_rem(&mut self) -> Result<(), String> {
        self.arg2_f64()
            .and_then(|(x, y)| {
                if y == 0f64 { Err("division by zero".to_owned()) } else { Ok((x, y)) }
            })
            .map(|(x,y)| {
                let div = x.div(y).floor();
                let rem = x % y;
                self.push(StackValue::Number(rem));
                self.push(StackValue::Number(div));
            })
    }

    pub fn exp(&mut self) -> Result<(), String> {
        self.arg2_f64().map(|(x,y)| {
            self.push(StackValue::Number(x.powf(y)));
        })
    }

    pub fn sqrt(&mut self) -> Result<(), String> {
        self.arg1_f64().map(|x| {
            self.push(StackValue::Number(x.sqrt()));
        })
    }

    pub fn mod_exp(&mut self) -> Result<(), String> {
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
            .map(|(base, exponent, modulus)| {
                if modulus == 1 {
                    self.push(StackValue::Number(0f64));
                } else {
                    let mut c = 1;
                    for _e_prime in 0..=(exponent-1) {
                        c = (c * base) % modulus;
                    }
                    self.push(StackValue::Number(c as f64));
                }
            })
    }

    fn arg1_f64(&mut self) -> Result<f64, String> {
        if self.stack.len() >= 1 {
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok( x )
        } else {
            Err("stack empty!".to_owned())
        }
    }

    fn arg2_f64(&mut self) -> Result<(f64, f64), String> {
        if self.stack.len() >= 2 {
            let StackValue::Number(y) = self.stack.pop_back().unwrap();
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok( (x, y) )
        } else {
            Err("stack empty!".to_owned())
        }
    }

    fn arg3_f64(&mut self) -> Result<(f64, f64, f64), String> {
        if self.stack.len() >= 3 {
            let StackValue::Number(z) = self.stack.pop_back().unwrap();
            let StackValue::Number(y) = self.stack.pop_back().unwrap();
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok( (x, y, z) )
        } else {
            Err("stack empty!".to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::stack::*;

    #[test]
    fn test_new() {
        let stack = Stack::new();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.precision, 0);
        assert_eq!(stack.input_radix, 10);
        assert_eq!(stack.output_radix, 10);
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        assert_eq!(stack.stack.len(), 0);
        stack.push(StackValue::Number(42.0));
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(42.0));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(42.0));
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(stack.pop(), Some(StackValue::Number(42.0)));
        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn test_pop_empty() {
        let mut stack = Stack::new();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(42.0));
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(stack.peek(), Some(&StackValue::Number(42.0)));
        assert_eq!(stack.stack.len(), 1);
    }

    #[test]
    fn test_peek_empty() {
        let stack = Stack::new();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(1.0));
        stack.stack.push_back(StackValue::Number(2.0));
        stack.stack.push_back(StackValue::Number(3.0));

        let mut result = Vec::new();
        for StackValue::Number(i) in stack.iter() {
            result.push(*i);
        }

        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_clear() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(1.0));
        stack.stack.push_back(StackValue::Number(2.0));
        stack.stack.push_back(StackValue::Number(3.0));
        assert_eq!(stack.stack.len(), 3);
        stack.clear();
        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn test_clear_empty() {
        let mut stack = Stack::new();
        assert_eq!(stack.stack.len(), 0);
        stack.clear();
        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn test_reverse() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(4.0));
        stack.stack.push_back(StackValue::Number(5.0));
        stack.reverse();
        assert_eq!(stack.stack.len(), 2);
        assert_eq!(*stack.stack.front().unwrap(), StackValue::Number(5.0));
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_reverse_singleton() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(4.0));
        stack.reverse();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_add() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(4.0));
        stack.stack.push_back(StackValue::Number(5.0));
        stack.add();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(9.0));
    }

    #[test]
    fn test_sub() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(4.0));
        stack.stack.push_back(StackValue::Number(5.0));
        stack.sub();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(-1.0));
    }

    #[test]
    fn test_mul() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(4.0));
        stack.stack.push_back(StackValue::Number(5.0));
        stack.mul();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(20.0));
    }

    #[test]
    fn test_div() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(20.0));
        stack.stack.push_back(StackValue::Number(5.0));
        stack.div();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_modulo() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(10.0));
        stack.stack.push_back(StackValue::Number(6.0));
        stack.modulo();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(4.0));
    }

    #[test]
    fn test_div_rem() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(10.0));
        stack.stack.push_back(StackValue::Number(6.0));
        stack.div_rem();
        assert_eq!(stack.stack.len(), 2);
        assert_eq!(*stack.stack.front().unwrap(), StackValue::Number(4.0));
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(1.0));
    }

    #[test]
    fn test_exp() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(2.0));
        stack.stack.push_back(StackValue::Number(10.0));
        stack.exp();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(1024.0));
    }

    #[test]
    fn test_sqrt() {
        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(10_000.0));
        stack.sqrt();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(100.0));
    }

    #[test]
    fn test_mod_exp() {
        // https://en.wikipedia.org/wiki/Modular_exponentiation

        let mut stack = Stack::new();
        stack.stack.push_back(StackValue::Number(4.0));
        stack.stack.push_back(StackValue::Number(13.0));
        stack.stack.push_back(StackValue::Number(497.0));
        stack.mod_exp();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(445.0));
    }

    #[test]
    fn test_get_input_radix() {
        let mut stack = Stack::new();
        stack.get_input_radix();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(stack.input_radix as f64));
    }

    #[test]
    fn test_get_output_radix() {
        let mut stack = Stack::new();
        stack.get_output_radix();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(stack.output_radix as f64));
    }

    #[test]
    fn test_get_precision() {
        let mut stack = Stack::new();
        stack.get_precision();
        assert_eq!(stack.stack.len(), 1);
        assert_eq!(*stack.stack.back().unwrap(), StackValue::Number(stack.precision as f64));
    }

    #[test]
    fn test_set_input_radix() {
        let mut stack = Stack::new();
        stack.push(StackValue::Number(42f64));
        stack.set_input_radix();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.input_radix, 42);
        assert_eq!(stack.output_radix, 10);
        assert_eq!(stack.precision, 0);
    }

    #[test]
    fn test_set_output_radix() {
        let mut stack = Stack::new();
        stack.push(StackValue::Number(42f64));
        stack.set_output_radix();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.input_radix, 10);
        assert_eq!(stack.output_radix, 42);
        assert_eq!(stack.precision, 0);
    }

    #[test]
    fn test_set_precision() {
        let mut stack = Stack::new();
        stack.push(StackValue::Number(42f64));
        stack.set_precision();
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.input_radix, 10);
        assert_eq!(stack.output_radix, 10);
        assert_eq!(stack.precision, 42);
    }
}