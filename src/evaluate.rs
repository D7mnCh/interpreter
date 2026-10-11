/*
- evaluate() function produce a value from an expr, you'll return the type of that value,
that type is one of lox's 4 types(number,string,bool,nil), so make an enum that represent
one of those 4 types
*/

use super::ast::Expr;
use crate::ast::{Op, UnaryOp};

enum LoxType {
    Number(f32),
    String(String),
    Bool(bool),
    Nil,
}

// use ADT to throw error with info
enum RuntimeError {
    BinaryOpMismatedTypes,
    BinaryOpDividedByZero,
    UnaryOpInvalidNumber,
    UnaryOpInvalidBool,
}

impl LoxType {
    fn evaluate(expr: Expr) -> Result<LoxType, RuntimeError> {
        return match expr {
            Expr::NumLiter(liter) => Ok(LoxType::Number(liter)),
            Expr::StringLiter(liter) => Ok(LoxType::String(liter)),
            Expr::BoolLiter(liter) => Ok(LoxType::Bool(liter)),
            Expr::Nil => Ok(LoxType::Nil),
            Expr::Unary { op, right } => Self::handle_unary_op(op, right),
            Expr::Binary { left, op, right } => Self::handle_binary_op(left, op, right),
            // need recusive cuz i need LoxType's varient, not an Expr
            Expr::Grouping(expr) => LoxType::evaluate(*expr),
        };
    }

    // NOTE maybe make this as a method on Op ?
    fn handle_binary_op(
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    ) -> Result<LoxType, RuntimeError> {
        let right = LoxType::evaluate(*right)?;
        let left = LoxType::evaluate(*left)?;

        match op {
            // arithmitic, operation between number and number
            Op::Sub => {
                if !Self::binary_op_are_both_numbers(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_numbers(right, left);
                Ok(LoxType::Number(left - right))
            }
            Op::Mul => {
                if !Self::binary_op_are_both_numbers(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_numbers(right, left);
                Ok(LoxType::Number(left * right))
            }
            Op::Div => {
                if !Self::binary_op_are_both_numbers(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_numbers(right, left);
                if right.is_nan() {
                    return Err(RuntimeError::BinaryOpDividedByZero);
                }
                Ok(LoxType::Number(left / right))
            }
            Op::Add => {
                if Self::binary_op_are_both_numbers(&left, &right) {
                    let (left, right) = Self::extracts_binary_operands_numbers(right, left);
                    return Ok(LoxType::Number(left + right));
                } else if Self::binary_op_are_both_strings(&left, &right) {
                    let (left, right) = Self::extracts_binary_operands_strings(right, left);
                    return Ok(LoxType::String(left + &right));
                } else {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
            }

            // logical op, operation between bool and bool
            Op::Lt => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left < right))
            }
            Op::Le => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left <= right))
            }
            Op::Gt => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left > right))
            }
            Op::Ge => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left >= right))
            }
            Op::EqEq => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left == right))
            }
            Op::Ne => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left != right))
            }
            Op::And => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }

                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left && right))
            }
            Op::Or => {
                if !Self::binary_op_are_both_same_types(&left, &right) {
                    return Err(RuntimeError::BinaryOpMismatedTypes);
                }
                let (left, right) = Self::extracts_binary_operands_bools(right, left);
                Ok(LoxType::Bool(left || right))
            }
        }
    }

    fn handle_unary_op(op: UnaryOp, right: Box<Expr>) -> Result<LoxType, RuntimeError> {
        let right = LoxType::evaluate(*right)?;

        match op {
            UnaryOp::Neg => match right {
                LoxType::Number(value) => Ok(LoxType::Number(-value)),
                _ => Err(RuntimeError::UnaryOpInvalidNumber),
            },

            UnaryOp::Not => match right {
                LoxType::Bool(value) => Ok(LoxType::Bool(!value)),
                _ => Err(RuntimeError::UnaryOpInvalidBool),
            },
        }
    }

    // NOTE maybe a review on doing unreachable!() ?
    fn extracts_binary_operands_numbers(right: LoxType, left: LoxType) -> (f32, f32) {
        let (left, right) = match (left, right) {
            (LoxType::Number(liter_left), LoxType::Number(liter_right)) => {
                (liter_left, liter_right)
            }
            _ => unreachable!(
                "must handle if both binary operands are numbers before calling \
                extracts_binary_operands_numbers()"
            ),
        };

        (left, right)
    }

    fn extracts_binary_operands_bools(right: LoxType, left: LoxType) -> (bool, bool) {
        let (left, right) = match (left, right) {
            (LoxType::Bool(liter_left), LoxType::Bool(liter_right)) => (liter_left, liter_right),
            _ => unreachable!(
                "must handle if both binary operands are numbers before calling \
                    extracts_binary_operands_numbers()"
            ),
        };

        (left, right)
    }

    fn extracts_binary_operands_strings(right: LoxType, left: LoxType) -> (String, String) {
        let (left, right) = match (left, right) {
            (LoxType::String(liter_left), LoxType::String(liter_right)) => {
                (liter_left, liter_right)
            }
            _ => unreachable!(
                "must handle if both binary operands are strings before calling \
                extracts_binary_operands_strings()"
            ),
        };

        (left, right)
    }

    fn binary_op_are_both_numbers(left: &LoxType, right: &LoxType) -> bool {
        matches!((left, right), (LoxType::Number(_), LoxType::Number(_)))
    }

    fn binary_op_are_both_strings(left: &LoxType, right: &LoxType) -> bool {
        matches!((left, right), (LoxType::String(_), LoxType::String(_)))
    }
    fn binary_op_are_both_same_types(left: &LoxType, right: &LoxType) -> bool {
        matches!(
            (left, right),
            (LoxType::Bool(_), LoxType::Bool(_))
                | (LoxType::String(_), LoxType::String(_))
                | (LoxType::Nil, LoxType::Nil)
                | (LoxType::Number(_), LoxType::Number(_))
        )
    }
}
