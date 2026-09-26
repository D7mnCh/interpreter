/*
- TODO turns this formal grammar rust impl using enums
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

// can't do Expr<T> cuz the recursion of Expr when choosing Literal(T) varient, all will have the
//the same T value(and you don't want that)
#[derive(Debug)]
pub enum Expr {
    NumLiter(f64),
    StringLiter(String),
    BoolLiter(bool),
    Nil,
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>), /* ( ) */
}

impl Expr {
    // Constructors to simplfy and hide some details when creating an instance of Expr (e.g Box::new())
    pub fn num_liter(value: f64) -> Expr {
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
    pub fn binary(right: Expr, op: Op, left: Expr) -> Expr {
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
    // pretty printer is going to represent AST rather then valid rlox(programing langauge) syntax
    fn pretty(&self) -> String {
        match self {
            Expr::NumLiter(value) => format!("{value}"),
            Expr::BoolLiter(value) => format!("{value}"),
            Expr::StringLiter(value) => value.clone(),
            Expr::Nil => "nil".to_string(),
            Expr::Binary { right, op, left } => {
                format!("({1} {0} {2})", right.pretty(), op.as_str(), left.pretty())
            }
            Expr::Unary { right, op } => format!("({1} {0})", right.pretty(), op.as_str()),
            Expr::Grouping(expr) => format!("(group {})", expr.pretty()),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Op {
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

impl Op {
    fn as_str(&self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Lt => "<",
            Op::Le => "<=",
            Op::Gt => ">",
            Op::Ge => ">=",
            Op::EqEq => "==",
            Op::Ne => "!=",
            Op::And => "and",
            Op::Or => "or",
        }
    }
}

#[test]
fn creating_an_exprs() {
    // (+ 1 1) -> 1 + 1
    // as you notice, it's kinda look like lisp :O

    let expr_1 = Expr::binary(Expr::num_liter(1.), Op::Add, Expr::num_liter(1.));
    assert_eq!(expr_1.pretty(), "(+ 1 1)".to_string());

    // (/ (group (+ 1 1)) 2) -> (1 + 1) / 2
    let expr_2 = Expr::binary(
        Expr::grouping(Expr::binary(
            Expr::num_liter(1.),
            Op::Add,
            Expr::num_liter(1.),
        )),
        Op::Div,
        Expr::num_liter(2.),
    );
    assert_eq!(expr_2.pretty(), "(/ (group (+ 1 1)) 2)".to_string());
}
