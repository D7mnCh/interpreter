use std::{env, io};
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 0 {
        println!("Usage: rlox [script.rlox]");
    } else if args.len() == 1 {
        run_file(args[1].clone());
    } else {
        run_prompt();
    }
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
    let scanner = Scanner::new();

    for token in tokens {
        println!("{token}");
    }
}
