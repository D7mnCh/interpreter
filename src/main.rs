use interpreter::{ast::Stmt, interpreter::Interpreter, parser::Parser, scanner::Scanner};
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
            println!("Usage: cargo run -- file.rlox");
        } else {
            let source = fs::read_to_string(args[1].clone())?;
            run(source);
        }
    }
    Ok(())
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

    let token_types_dbg: Vec<&str> = tokens
        .iter()
        .map(|token| token.get_token_type().as_str())
        .collect();
    println!("tokens -> {token_types_dbg:?}");

    let mut parser = Parser::new(tokens);
    // can do filter_map(|stmt| stmt) that magically discrad None and do the same thing with flatten()
    // flatten() remove Option nesting, and magically discard None
    let stmts: Vec<Stmt> = parser.parse();
    println!("parser -> {:?}", stmts);

    Interpreter::interpret(stmts);
}
