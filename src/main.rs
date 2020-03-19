use clap::{App, Arg, ArgMatches};
use dc::calculator::{Calculator, OpResult};
use dc::process_input;

fn main() {
    let matches = parse_arguments();
    let mut calculator = Calculator::new();

    if let Some(value) = matches.value_of("expression") {
        process_input(&mut calculator, value).unwrap();
    } else if let Some(value) = matches.value_of("file") {
        let content = std::fs::read_to_string(value).unwrap();
        process_input(&mut calculator, &content).unwrap();
    } else if let Some(files) = matches.values_of("FILE") {
        for file in files {
            let content = std::fs::read_to_string(file).unwrap();
            process_input(&mut calculator, &content).unwrap();
        }
    } else {
        repl(&mut calculator);
    }
}

fn parse_arguments() -> ArgMatches<'static> {
    App::new("dc")
        .version("0.1")
        .author("Ferenc Nagy <nagy.ferenc.jr@protonmail.com>")
        .about("Clone of the Unix program called dc")
        .arg(Arg::with_name("expression")
            .short("e")
            .long("expression")
            .value_name("scriptexpression")
            .help("Add the commands in script to the set of commands to be run while processing the input.")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("script-file")
            .help("Add the commands contained in the file script-file to the set of commands to be run while processing the input.")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("FILE")
            .help("any files to process one-by-one")
            .multiple(true)
            .required(false))
        .get_matches()
}

fn repl(mut calculator: &mut Calculator) {
    let stdin = std::io::stdin();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        match process_input(&mut calculator, buffer.trim().as_ref()) {
            Ok(result) => match result {
                OpResult::Exit => break,
                _ => (),
            },
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        };
    }
}
