use interpreter::{evaluate::LoxValue, parser::Parser, scanner::Scanner};
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

    // TODO impl from Tokens to get token_types
    let mut token_types: Vec<&str> = Vec::new();
    for token in tokens.clone().into_iter() {
        let token_type = token.get_token_type().as_str();
        token_types.push(token_type);
    }
    println!("tokens -> {token_types:?}");

    let mut parser = Parser::new(tokens);
    let expr_kind = parser.parse();

    if let Some(expr) = expr_kind {
        println!("expr -> {}", expr.pretty());
        if let Some(lox_value) = LoxValue::interpret(expr) {
            println!("lox_value -> {}", lox_value.pretty());
        }
    }
}
