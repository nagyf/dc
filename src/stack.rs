use std::collections::LinkedList;
use std::fmt;
use std::collections::linked_list::Iter;
use std::ops::{Rem, Div};

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

    fn arg2_f64(&mut self) -> Result<(f64, f64), String> {
        if self.stack.len() >= 2 {
            let StackValue::Number(y) = self.stack.pop_back().unwrap();
            let StackValue::Number(x) = self.stack.pop_back().unwrap();
            Ok( (x, y) )
        } else {
            Err("stack empty!".to_owned())
        }
    }
}