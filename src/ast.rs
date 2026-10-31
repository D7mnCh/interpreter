/*
expression     → literal
               | unary
               | binary
               | grouping ;
literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
*/

use crate::scanner::Token;
use std::fmt::Debug;

// can't do Expr<T> cuz the recursion of Expr when choosing Literal(T) varient, all will have the
//the same T value(and you don't want that)
#[derive(Debug, Clone)]
// Expr == primary rule from the grammar graph
pub enum Expr {
    NumLiter(f32),
    StringLiter(String),
    BoolLiter(bool),
    // Identifier(Token) used to get the value of that ident from that Token
    Identifier(Token),
    Nil,
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>), /* ( ) */
}

impl Expr {
    // Constructors to simplfy and hide some details when creating an instance of Expr (e.g Box::new())
    pub fn num_liter(value: f32) -> Expr {
        Expr::NumLiter(value)
    }

    pub fn bool_liter(value: bool) -> Expr {
        Expr::BoolLiter(value)
    }

    pub fn string_liter(value: String) -> Expr {
        Expr::StringLiter(value)
    }

    pub fn nil() -> Expr {
        Expr::Nil
    }

    pub fn ident(token: Token) -> Expr {
        Expr::Identifier(token)
    }

    pub fn binary(left: Expr, op: BinaryOp, right: Expr) -> Expr {
        Expr::Binary {
            right: right.into(),
            op,
            left: left.into(),
        }
    }
    pub fn unary(op: UnaryOp, right: Expr) -> Expr {
        Expr::Unary {
            right: right.into(),
            op,
        }
    }
    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(expr.into())
    }
}

impl Expr {
    // pretty printer is going to represent AST in a pretty way
    pub fn pretty(&self) -> String {
        match self {
            Expr::NumLiter(value) => format!("{value}"),
            Expr::BoolLiter(value) => format!("{value}"),
            Expr::StringLiter(value) => value.clone(),
            Expr::Nil => "nil".to_string(),
            Expr::Identifier(token) => format!("{}", token.get_lexeme()),
            Expr::Binary { right, op, left } => {
                format!(
                    "({op} {left} {right})",
                    right = right.pretty(),
                    op = op.as_str(),
                    left = left.pretty()
                )
            }
            Expr::Unary { right, op } => format!("({1} {0})", right.pretty(), op.as_str()),
            Expr::Grouping(expr) => format!("(group {})", expr.pretty()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

impl UnaryOp {
    fn as_str(&self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
        }
    }
}

#[derive(Debug, Clone)]
// contains arithimitc, logical and comparison operators
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    Lt,
    Le,
    Gt,
    Ge,
    EqEq,
    Ne,
    And,
    Or,
}

impl BinaryOp {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::EqEq => "==",
            Self::Ne => "!=",
            Self::And => "and",
            Self::Or => "or",
        }
    }
}

#[derive(Clone)]
pub enum Stmt {
    // exprStmt
    Expr(Expr),
    Print(Expr),
    VarDecl { name: String, expr: Expr },
}

impl Stmt {
    pub fn expr(expr: Expr) -> Stmt {
        Self::Expr(expr)
    }
    pub fn print(expr: Expr) -> Stmt {
        Self::Print(expr)
    }
    pub fn var_decl(name: String, expr: Expr) -> Stmt {
        Self::VarDecl { name, expr }
    }
}

// pretty printer
impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(expr) => f.debug_tuple("Expr").field(&expr.pretty()).finish(),
            Self::Print(expr) => f.debug_tuple("Print").field(&expr.pretty()).finish(),
            Self::VarDecl {
                name: var_name,
                expr,
            } => f
                .debug_tuple("VarDecl")
                .field(&var_name)
                .field(&expr.pretty())
                .finish(),
        }
    }
}

#[test]
fn creating_an_exprs() {
    // (+ 1 1) -> 1 + 1
    // as you notice, it's kinda look like lisp :O

    let expr_1 = Expr::binary(Expr::num_liter(1.), BinaryOp::Add, Expr::num_liter(1.));
    assert_eq!(expr_1.pretty(), "(+ 1 1)".to_string());

    // (/ (group (+ 1 1)) 2) -> (1 + 1) / 2
    let expr_2 = Expr::binary(
        Expr::grouping(Expr::binary(
            Expr::num_liter(1.),
            BinaryOp::Add,
            Expr::num_liter(1.),
        )),
        BinaryOp::Div,
        Expr::num_liter(2.),
    );
    assert_eq!(expr_2.pretty(), "(/ (group (+ 1 1)) 2)".to_string());
}
