use std::collections::LinkedList;
use std::fmt;
use std::collections::linked_list::Iter;

#[derive(Debug, Copy, Clone)]
pub enum StackValue {
    Integer(i64)
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackValue::Integer(x) => write!(f, "{}", x)
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
        self.arg2_i64().map(|(x,y)| {
            self.push(StackValue::Integer(x + y));
        })
    }

    pub fn sub(&mut self) -> Result<(), String> {
        self.arg2_i64().map(|(x,y)| {
            self.push(StackValue::Integer(x - y));
        })
    }

    pub fn mul(&mut self) -> Result<(), String> {
        self.arg2_i64().map(|(x,y)| {
            self.push(StackValue::Integer(x * y));
        })
    }

    pub fn div(&mut self) -> Result<(), String> {
        self.arg2_i64()
            .and_then(|(x, y)| {
                if y == 0 { Err("division by zero".to_owned()) } else { Ok((x, y)) }
            })
            .map(|(x,y)| {
                self.push(StackValue::Integer(x / y));
            })
    }

    pub fn modulo(&mut self) -> Result<(), String> {
        self.arg2_i64().map(|(x,y)| {
            self.push(StackValue::Integer(x % y));
        })
    }

    pub fn exp(&mut self) -> Result<(), String> {
        self.arg2_i64().map(|(x,y)| {
            self.push(StackValue::Integer(x.pow(y as u32)));
        })
    }

    fn arg2_i64(&mut self) -> Result<(i64, i64), String> {
        if self.stack.len() >= 2 {
            let StackValue::Integer(y) = self.stack.pop_back().unwrap();
            let StackValue::Integer(x) = self.stack.pop_back().unwrap();
            Ok( (x, y) )
        } else {
            Err("stack empty!".to_owned())
        }
    }
}