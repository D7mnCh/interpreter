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
enum Expr {
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
    Grouping(Box<Expr>), // -> ( )
}

impl Expr {
    // Constructors to simplfy and hide some details when creating an instance of Expr (e.g Box::new())
    fn num_liter(value: f64) -> Expr {
        Expr::NumLiter(value)
    }
    fn bool_liter(value: bool) -> Expr {
        Expr::BoolLiter(value)
    }
    fn string_liter (value: String) -> Expr{
        Expr::StringLiter(value)
    }
    fn nil() -> Expr {
        Expr::Nil
    }
    fn binary(right: Expr, op: Op, left: Expr) -> Expr {
        Expr::Binary {
            right: Box::new(right),
            op,
            left: Box::new(left)
        }
    }
    fn unary(op: UnaryOp, right: Expr) -> Expr{
        Expr::Unary {
            right: Box::new(right),
            op,
        }
    }
    fn grouping (expr: Expr) -> Expr{
        Expr::Grouping (Box::new(expr))
    }
}

#[derive(Debug)]
enum UnaryOp {
    Neg,
    Not
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
    LT,
    LE,
    GT,
    GE,
    Eq,
    And,
    Or,
    Neg,
}

// TODO impl pretty method for Expr, that turns the Expr instance into a String, used for debugging

#[test]
fn creating_an_expr() {
    // 1 + 1
    let expr_2 = Expr::binary(1, Op::Add, 1);
}
