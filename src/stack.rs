use std::collections::LinkedList;
use std::fmt;
use std::collections::linked_list::Iter;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
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
    stack: LinkedList<StackValue>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: LinkedList::new()
        }
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