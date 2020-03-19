use num_bigint::BigInt;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    DivRem,
    Exp,
    Sqrt,
    ModExp,
    PrintPop,
    PrintPeek,
    PrintAll,
    Push(BigInt),
    Exit,

    // Stack operations
    Clear,
    Duplicate,
    Reverse,
    SetInputRadix,
    SetOutputRadix,
    SetPrecision,
    GetInputRadix,
    GetOutputRadix,
    GetPrecision,
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
            // Stack operations
            'c' => tokens.push(Op::Clear),
            'd' => tokens.push(Op::Duplicate),
            'r' => tokens.push(Op::Reverse),

            'i' => tokens.push(Op::SetInputRadix),
            'o' => tokens.push(Op::SetOutputRadix),
            'k' => tokens.push(Op::SetPrecision),
            'I' => tokens.push(Op::GetInputRadix),
            'O' => tokens.push(Op::GetOutputRadix),
            'K' => tokens.push(Op::GetPrecision),

            // Calculator operations
            'q' => tokens.push(Op::Exit),
            'p' => tokens.push(Op::PrintPeek),
            'n' => tokens.push(Op::PrintPop),
            'f' => tokens.push(Op::PrintAll),
            '+' => tokens.push(Op::Add),
            '-' => tokens.push(Op::Sub),
            '*' => tokens.push(Op::Mul),
            '/' => tokens.push(Op::Div),
            '%' => tokens.push(Op::Mod),
            '~' => tokens.push(Op::DivRem),
            '^' => tokens.push(Op::Exp),
            'v' => tokens.push(Op::Sqrt),
            '|' => tokens.push(Op::ModExp),
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

                // TODO radix
                if let Some(num) = BigInt::parse_bytes(num_str.as_bytes(), 10) {
                    tokens.push(Op::Push(num));
                } else {
                    Err(format!("Unable to parse number: {} with radix: {}", ch, 10))?;
                }
            },
            _ => {
                return Err(format!("Unknown operation: {}", ch))
            }
        };
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use crate::token::*;

    #[test]
    fn tokenize_operation() {
        assert_eq!(tokenize("c").unwrap(), vec![Op::Clear]);
        assert_eq!(tokenize("d").unwrap(), vec![Op::Duplicate]);
        assert_eq!(tokenize("r").unwrap(), vec![Op::Reverse]);

        assert_eq!(tokenize("i").unwrap(), vec![Op::SetInputRadix]);
        assert_eq!(tokenize("o").unwrap(), vec![Op::SetOutputRadix]);
        assert_eq!(tokenize("k").unwrap(), vec![Op::SetPrecision]);
        assert_eq!(tokenize("I").unwrap(), vec![Op::GetInputRadix]);
        assert_eq!(tokenize("O").unwrap(), vec![Op::GetOutputRadix]);
        assert_eq!(tokenize("K").unwrap(), vec![Op::GetPrecision]);

        assert_eq!(tokenize("q").unwrap(), vec![Op::Exit]);
        assert_eq!(tokenize("p").unwrap(), vec![Op::PrintPeek]);
        assert_eq!(tokenize("n").unwrap(), vec![Op::PrintPop]);
        assert_eq!(tokenize("f").unwrap(), vec![Op::PrintAll]);
        assert_eq!(tokenize("+").unwrap(), vec![Op::Add]);
        assert_eq!(tokenize("-").unwrap(), vec![Op::Sub]);
        assert_eq!(tokenize("*").unwrap(), vec![Op::Mul]);
        assert_eq!(tokenize("/").unwrap(), vec![Op::Div]);
        assert_eq!(tokenize("%").unwrap(), vec![Op::Mod]);
        assert_eq!(tokenize("~").unwrap(), vec![Op::DivRem]);
        assert_eq!(tokenize("^").unwrap(), vec![Op::Exp]);
        assert_eq!(tokenize("v").unwrap(), vec![Op::Sqrt]);
        assert_eq!(tokenize("|").unwrap(), vec![Op::ModExp]);
    }

    #[test]
    fn tokenize_zero() {
        assert_eq!(tokenize("0").unwrap(), vec![Op::Push(BigInt::from(0))]);
    }

    #[test]
    fn tokenize_number() {
        assert_eq!(tokenize("42").unwrap(), vec![Op::Push(BigInt::from(42))]);
    }

    #[test]
    fn tokenize_negative_number() {
        assert_eq!(tokenize("_42").unwrap(), vec![Op::Push(BigInt::from(-42))]);
    }

    // TODO
    // #[test]
    // fn tokenize_floating_number() {
    //     assert_eq!(tokenize("3.1415").unwrap(), vec![Op::Push(BigInt::from(3.1415))]);
    // }
    //
    // #[test]
    // fn tokenize_negative_floating_number() {
    //     assert_eq!(tokenize("_3.1415").unwrap(), vec![Op::Push(BigInt::from(-3.1415))]);
    // }

    #[test]
    fn tokenize_leading_zero_number() {
        assert_eq!(tokenize("04").unwrap(), vec![Op::Push(BigInt::from(4))]);
    }

    #[test]
    fn tokenize_unknown_operation() {
        assert_eq!(tokenize("x"), Err("Unknown operation: x".to_owned()));
    }

    #[test]
    fn tokenize_multiple() {
        let expected = vec![
            Op::Push(BigInt::from(42)),
            Op::Duplicate,
            Op::Mul,
            Op::PrintPeek,
            Op::Exit
        ];
        assert_eq!(tokenize("42d*pq"), Ok(expected));
    }
}