use crate::{
    ast::Stmt,
    evaluate::{LoxValue, RuntimeError},
};

pub struct Interpreter;

impl Interpreter {
    // program rule
    pub fn interpret(statements: Vec<Stmt>) {
        for statement in statements {
            let _ = Self::execute(statement).inspect_err(|why| why.report());
        }
    }

    fn execute(stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => {
                LoxValue::evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let expr = LoxValue::evaluate(expr)?;
                println!("{}", expr.pretty());
                Ok(())
            }
        }
    }
}
