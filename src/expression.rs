use core::fmt;

use crate::parser::{AstNode, BinaryOperationType, UnaryOperationType};


#[derive(Debug)]
pub enum ComputationError {
    DivideByZeroError(DivideByZeroError)
}


impl fmt::Display for ComputationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputationError::DivideByZeroError(divide_by_zero_error) => write!(f, "{}", divide_by_zero_error.message),
        }
    }
}


#[derive(Debug)]
pub struct DivideByZeroError {
    message: String
}


impl DivideByZeroError {
    fn new(message: &str) -> Self {
        DivideByZeroError { message: message.to_string() }
    }
}


pub fn execute(expression: &Box<AstNode>) -> Result<f64, ComputationError> {
    let root_node = expression;
    let current_node = root_node.as_ref();

    match current_node {
        AstNode::BinaryOperation(
            operation_type, 
            left, 
            right
        ) => compute_binary(operation_type, left, right),

        AstNode::UnaryOperation(
            operation_type, 
            operand
        ) => compute_unary(operation_type, operand),

        AstNode::Number(x) => Ok(*x),
    }
}


fn compute_unary(operation_type: &UnaryOperationType, operand: &Box<AstNode>) -> Result<f64, ComputationError> {
    match operation_type {
        UnaryOperationType::Negate => Ok(-1.0 * execute(operand)?),
    }
}


fn compute_binary(operation_type: &BinaryOperationType, left: &Box<AstNode>, right: &Box<AstNode>) -> Result<f64, ComputationError> {
    let left_side = execute(left)?;
    let right_side = execute(right)?;

    match operation_type {
        BinaryOperationType::Add => Ok(left_side + right_side),
        BinaryOperationType::Subtract => Ok(left_side - right_side),
        BinaryOperationType::Multiply => Ok(left_side * right_side),
        BinaryOperationType::Divide => {
            if right_side == 0.0 {
                return Err(ComputationError::DivideByZeroError(DivideByZeroError::new("Division by zero")))
            }
            Ok(left_side / right_side)
        },
        BinaryOperationType::Modulus => {
            if right_side == 0.0 {
                return Err(ComputationError::DivideByZeroError(DivideByZeroError::new("Division by zero")))
            }
            Ok(left_side % right_side)
        },
    }
}