#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    DivRem,
    Exp,
    Sqrt,
    Clear,
    Duplicate,
    PrintPop,
    PrintPeek,
    PrintAll,
    Push(f64),
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
            '*' => tokens.push(Op::Mul),
            '/' => tokens.push(Op::Div),
            '%' => tokens.push(Op::Mod),
            '~' => tokens.push(Op::DivRem),
            '^' => tokens.push(Op::Exp),
            'v' => tokens.push(Op::Sqrt),
            '-' => tokens.push(Op::Sub),
            '_' | '0'..='9' => {
                let mut num_str = String::new();
                num_str.push(ch);
                while index < length {
                    match buffer.chars().nth(index).unwrap() {
                        '.' | '0'..='9' => {
                            num_str.push(buffer.chars().nth(index).unwrap());
                            index += 1;
                        },
                        _ => break,
                    };
                }
                num_str = num_str.replace("_", "-");
                let num = num_str.parse::<f64>().unwrap();
                tokens.push(Op::Push(num));
            },
            _ => {
                return Err(format!("Unknown operation: {}", ch))
            }
        };
    }

    Ok(tokens)
}