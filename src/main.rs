use dc::stack::{Stack, StackValue};
use dc::{process_input, OpResult};

fn main() {
    let mut stack = Stack::new();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer);

        match process_input(&mut stack, buffer.trim().to_lowercase().as_ref()) {
            Ok(result) => {
                match result {
                    OpResult::Exit => break,
                    _ => ()
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        };
    }
}