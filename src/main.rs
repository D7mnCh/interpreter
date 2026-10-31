use interpreter::{
    ast::Stmt, evaluate::Environment, interpreter::Interpreter, parser::Parser, scanner::Scanner,
};
use std::{
    env, fs,
    io::{self, Result},
};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    /*
    env should be at runtime without dropping, constructed before run (especially
    before run_prompt(REPL)) cuz env needs to live as long as the program
    */
    let mut env = Environment::default();
    if args.len() == 1 {
        run_prompt(&mut env);
    } else if args.len() == 2 {
        if args[1] == "--help" {
            println!("Usage: cargo run -- file.rlox");
        } else {
            let source = fs::read_to_string(args[1].clone())?;
            run(source, &mut env);
        }
    }
    Ok(())
}

fn run_prompt(mut env: &mut Environment) {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Err(err) => {
                eprintln!("{err}");
                break;
            }

            Ok(_) => run(line, &mut env),
        }
    }
}

fn run(source: String, env: &mut Environment) {
    let source_as_chars = &source.chars().collect::<Vec<char>>();
    let mut scanner = Scanner::new(&source_as_chars);
    let tokens = scanner.tokenize();

    let token_types_dbg: Vec<&str> = tokens
        .iter()
        .map(|token| token.get_token_type().as_str())
        .collect();
    println!("tokens -> {token_types_dbg:?}");

    let mut parser = Parser::new(tokens);
    let stmts: Vec<Stmt> = parser.parse();
    println!("parser -> {:?}", stmts);

    let mut interpreter = Interpreter::new(env);
    interpreter.interpret(stmts);
}
