mod scanner;
use scanner::Scanner;

use std::{
    env, fs,
    io::{self, Result},
};
// TODO i must add logging
fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: rlox file.rlox");
    } else if args.len() == 2 {
        let source = fs::read_to_string(args[1].clone())?;
        run_file(source);
    } else {
        run_prompt();
    }
    Ok(())
}

fn run_file(source: String) {
    run(source);
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Err(err) => {
                eprintln!("{err}");
                break;
            }

            Ok(_) => run(line),
        }
    }
}

fn run(source: String) {
    let source_as_chars = &source.chars().collect::<Vec<char>>();
    let mut scanner = Scanner::new(&source_as_chars);
    let tokens = scanner.tokenize();

    for token in tokens {
        match token {
            Ok(token) => println!("{token:?}"),
            Err(err) => println!("{err:?}"),
        }
    }
}
