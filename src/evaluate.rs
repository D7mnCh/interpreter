// TODO: tests
// TODO: forget, need a code review

/*
- evaluate() function produce a value from an expr, you'll return the type of that value,
that type is one of lox's 4 types(number,string,bool,nil), so make an enum that represent
one of those 4 types
*/

use crate::ast::{BinaryOp, Expr, UnaryOp};

pub enum RuntimeError {
    BinaryOpInvalidArithmticOpaeration,
    BinaryOpInvalidComparison,
    BinaryOpDividedByZero,
    BinaryOpMustBothBeBool,
    UnaryOpInvalidNumber,
    UnaryOpMustBeBool,
    BinaryOpOperandsMustBeBool,
}

impl RuntimeError {
    // TODO: expand error msg
    pub fn report(&self) {
        match self {
            RuntimeError::BinaryOpInvalidArithmticOpaeration => {
                println!("[Runtime Error]: Invalid arithmetic operation")
            }
            RuntimeError::BinaryOpDividedByZero => println!("[Runtime Error]: Diveded by zero"),
            RuntimeError::UnaryOpInvalidNumber => println!("[Runtime Error]: Expected numbers"),
            RuntimeError::UnaryOpMustBeBool => println!("[Runtime Error]: Expected bool"),
            RuntimeError::BinaryOpMustBothBeBool => {
                println!("[Runtime Error]: Expected both to be bool")
            }
            RuntimeError::BinaryOpInvalidComparison => {
                println!("[Runtime Error]: Invalid comparison")
            }
            RuntimeError::BinaryOpOperandsMustBeBool => {
                println!("[Runtime Error]: Operands must evaluate to bool")
            }
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum LoxValue {
    Number(f32),
    String(String),
    Bool(bool),
    Nil,
}

impl LoxValue {
    pub fn evaluate(expr: Expr) -> Result<LoxValue, RuntimeError> {
        return match expr {
            Expr::NumLiter(liter) => Ok(LoxValue::Number(liter)),
            Expr::StringLiter(liter) => Ok(LoxValue::String(liter)),
            Expr::BoolLiter(liter) => Ok(LoxValue::Bool(liter)),
            Expr::Nil => Ok(LoxValue::Nil),
            Expr::Unary { op, right } => Self::handle_unary_op(op, right),
            Expr::Binary { left, op, right } => Self::handle_binary_op(left, op, right),
            // need recusive cuz i need LoxType's varient, not an Expr
            Expr::Grouping(expr) => LoxValue::evaluate(*expr),
        };
    }

    fn handle_binary_op(
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    ) -> Result<LoxValue, RuntimeError> {
        // i should begin with left for some reason
        let left = LoxValue::evaluate(*left)?;
        let right = LoxValue::evaluate(*right)?;

        /*
        i was checking l and r on a seperate function, and extract in another function
        that leads to unnessesery unreachable!() (cuz used two match expr), and the extraction
        function is useless cuz i did the extraction of enum's variant value in other function rather
        then in place (like what -.-)
        */
        match op {
            // handle arithmitic operations
            BinaryOp::Sub => match (left, right) {
                (LoxValue::Number(left), LoxValue::Number(right)) => {
                    Ok(LoxValue::Number(left - right))
                }
                _ => return Err(RuntimeError::BinaryOpInvalidArithmticOpaeration),
            },
            BinaryOp::Mul => match (left, right) {
                (LoxValue::Number(left), LoxValue::Number(right)) => {
                    Ok(LoxValue::Number(left * right))
                }
                _ => return Err(RuntimeError::BinaryOpInvalidArithmticOpaeration),
            },

            BinaryOp::Div => match (left, right) {
                (LoxValue::Number(_), LoxValue::Number(right)) if right == 0. => {
                    return Err(RuntimeError::BinaryOpDividedByZero);
                }
                (LoxValue::Number(left), LoxValue::Number(right)) => {
                    Ok(LoxValue::Number(left / right))
                }
                _ => return Err(RuntimeError::BinaryOpInvalidArithmticOpaeration),
            },
            BinaryOp::Add => match (left, right) {
                (LoxValue::Number(left), LoxValue::Number(right)) => {
                    Ok(LoxValue::Number(left + right))
                }
                (LoxValue::String(left), LoxValue::String(right)) => {
                    Ok(LoxValue::String(left + &right))
                }
                _ => return Err(RuntimeError::BinaryOpInvalidArithmticOpaeration),
            },

            // handle comparison operations
            BinaryOp::Lt => {
                if matches!(
                    (&left, &right),
                    (LoxValue::String(_), LoxValue::String(_),)
                        | (LoxValue::Number(_), LoxValue::Number(_))
                ) {
                    return Ok(LoxValue::Bool(left < right));
                }
                return Err(RuntimeError::BinaryOpInvalidComparison);
            }
            BinaryOp::Le => {
                if matches!(
                    (&left, &right),
                    (LoxValue::String(_), LoxValue::String(_),)
                        | (LoxValue::Number(_), LoxValue::Number(_))
                ) {
                    return Ok(LoxValue::Bool(left <= right));
                }
                return Err(RuntimeError::BinaryOpInvalidComparison);
            }
            BinaryOp::Gt => {
                if matches!(
                    (&left, &right),
                    (LoxValue::String(_), LoxValue::String(_),)
                        | (LoxValue::Number(_), LoxValue::Number(_))
                ) {
                    return Ok(LoxValue::Bool(left > right));
                }
                return Err(RuntimeError::BinaryOpInvalidComparison);
            }
            BinaryOp::Ge => {
                if matches!(
                    (&left, &right),
                    (LoxValue::String(_), LoxValue::String(_),)
                        | (LoxValue::Number(_), LoxValue::Number(_))
                ) {
                    return Ok(LoxValue::Bool(left >= right));
                }
                return Err(RuntimeError::BinaryOpInvalidComparison);
            }
            BinaryOp::EqEq => {
                if matches!(
                    (&left, &right),
                    (LoxValue::String(_), LoxValue::String(_),)
                        | (LoxValue::Number(_), LoxValue::Number(_))
                ) {
                    return Ok(LoxValue::Bool(left == right));
                }
                return Err(RuntimeError::BinaryOpInvalidComparison);
            }
            BinaryOp::Ne => {
                if matches!(
                    (&left, &right),
                    (LoxValue::Bool(_), LoxValue::Bool(_))
                        | (LoxValue::String(_), LoxValue::String(_),)
                        | (LoxValue::Number(_), LoxValue::Number(_))
                ) {
                    return Ok(LoxValue::Bool(left != right));
                }
                return Err(RuntimeError::BinaryOpInvalidComparison);
            }

            // logical binary operations
            BinaryOp::And => match (left, right) {
                (LoxValue::Bool(left), LoxValue::Bool(right)) => Ok(LoxValue::Bool(left && right)),
                _ => return Err(RuntimeError::BinaryOpOperandsMustBeBool),
            },
            BinaryOp::Or => match (left, right) {
                (LoxValue::Bool(left), LoxValue::Bool(right)) => Ok(LoxValue::Bool(left || right)),
                _ => return Err(RuntimeError::BinaryOpOperandsMustBeBool),
            },
        }
    }

    fn handle_unary_op(op: UnaryOp, right: Box<Expr>) -> Result<LoxValue, RuntimeError> {
        let right = LoxValue::evaluate(*right)?;

        match op {
            UnaryOp::Neg => match right {
                LoxValue::Number(value) => Ok(LoxValue::Number(-value)),
                _ => Err(RuntimeError::UnaryOpInvalidNumber),
            },

            UnaryOp::Not => match right {
                LoxValue::Bool(value) => Ok(LoxValue::Bool(!value)),
                _ => Err(RuntimeError::UnaryOpMustBeBool),
            },
        }
    }
}

impl LoxValue {
    pub fn pretty(&self) -> String {
        match self {
            LoxValue::Number(liter) => format!("{liter}"),
            LoxValue::String(liter) => format!("{liter}"),
            LoxValue::Bool(liter) => format!("{liter}"),
            LoxValue::Nil => format!("Nil"),
        }
    }
}
