mod scanner;
use scanner::Scanner;

use std::{
    env, fs,
    io::{self, Result},
};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        if args[1] == "--help" {
            println!("Usage: rlox file.rlox");
        } else {
            let source = fs::read_to_string(args[1].clone())?;
            run_file(source);
        }
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

    for token in tokens.into_iter() {
        // show only valid tokens
        if let Some(ref _ty) = token.token_type {
            println!("{token:?}");
        }
    }
}
