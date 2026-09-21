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
// TODO impl Constructors to simplfy and hid some details when creating an instance of Expr (e.g Box)

#[test]
fn creating_an_expr() {
    // TODO use the new method with Constructors after impl theme
    // 1 + 1
    let expr_1 = Expr::Binary {
        left: Box::new(Expr::NumLiter(1.)),
        op: Op::Add,
        right: Box::new(Expr::Num_Liter(1.))
    };
}
