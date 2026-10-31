use crate::{
    ast::Stmt,
    evaluate::{Environment, LoxValue, RuntimeError},
};

pub struct Interpreter<'a> {
    env: &'a mut Environment,
}

impl<'a> Interpreter<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Interpreter { env }
    }
}

impl<'a> Interpreter<'a> {
    // program rule
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            let _ = self.execute(statement).inspect_err(|why| why.report());
        }
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => {
                LoxValue::evaluate(expr, &self.env)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let expr = LoxValue::evaluate(expr, &self.env)?;
                println!("{}", expr.pretty());
                Ok(())
            }
            Stmt::VarDecl {
                name: var_name,
                expr,
            } => {
                self.env.define_glob_var(&var_name, expr.to_owned());
                Ok(())
            }
        }
    }
}
