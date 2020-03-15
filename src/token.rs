#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Clear,
    Duplicate,
    PrintPop,
    PrintPeek,
    PrintAll,
    Push(i64),
    Exit
}

pub fn tokenize(str: &str) -> Result<Vec<Op>, String> {
    let mut tokens = Vec::new();
    let length = str.len();
    let buffer = str.to_owned();
    let mut index = 0;
    while index < length {
        let ch = buffer.chars().nth(index).unwrap();
        index += 1;

        match ch {
            'q' => tokens.push(Op::Exit),
            'p' => tokens.push(Op::PrintPeek),
            'n' => tokens.push(Op::PrintPop),
            'f' => tokens.push(Op::PrintAll),
            'c' => tokens.push(Op::Clear),
            'd' => tokens.push(Op::Duplicate),
            '+' => tokens.push(Op::Add),
            '-' => tokens.push(Op::Sub),
            '*' => tokens.push(Op::Mul),
            '/' => tokens.push(Op::Div),
            '%' => tokens.push(Op::Mod),
            '^' => tokens.push(Op::Exp),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut num_str = String::new();
                num_str.push(ch);
                while index < length {
                    match buffer.chars().nth(index).unwrap() {
                        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            num_str.push(buffer.chars().nth(index).unwrap());
                            index += 1;
                        },
                        _ => break,
                    };
                }
                tokens.push(Op::Push(num_str.parse::<i64>().unwrap()))
            },
            _ => {
                return Err(format!("Unknown operation: {}", ch))
            }
        };
    }

    Ok(tokens)
}