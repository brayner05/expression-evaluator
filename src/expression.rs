use core::fmt;

use crate::{parser::{AstNode, BinaryOperationType, UnaryOperationType}, pxpr};


#[derive(Debug)]
pub enum Value {
    Float(f64),
    Integer(i64),
    Boolean(bool)
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}


impl Value {
    fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(n) => Some(*n),
            Value::Integer(n) => Some(*n as f64),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}


pub fn execute(expression: &Box<AstNode>) -> Result<Value, pxpr::Error> {
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

        AstNode::Integer(x) => Ok(Value::Integer(*x)),
        AstNode::Boolean(x) => Ok(Value::Boolean(*x)),
        AstNode::Float(x) => Ok(Value::Float(*x)),
    }
}


///
/// Computes the result of a unary operation.
/// 
fn compute_unary(operation_type: &UnaryOperationType, operand: &Box<AstNode>) -> Result<Value, pxpr::Error> {
    let operand_value = execute(operand)?;
    match operation_type {
        UnaryOperationType::ArithmeticNegate => compute_arithmetic_negation(operand_value),
        UnaryOperationType::LogicalNot => compute_logical_not(operand_value),
        UnaryOperationType::BitwiseNot => compute_bitwise_not(operand_value)
    }
}


fn compute_bitwise_not(operand: Value) -> Result<Value, pxpr::Error> {
    match operand.as_integer() {
        Some(x) => Ok(Value::Integer(!x)),
        None => Err(pxpr::Error::new(0, format!("Invalid operand for '~': {}", operand))),
    }
}


fn compute_binary(
    operation_type: &BinaryOperationType,
    left: &Box<AstNode>, 
    right: &Box<AstNode>
) -> Result<Value, pxpr::Error> {
    let left_side = execute(left)?;
    let right_side = execute(right)?;

    match operation_type {
        BinaryOperationType::Add => compute_addition(&left_side, &right_side),
        BinaryOperationType::Subtract => compute_subtraction(&left_side, &right_side),
        BinaryOperationType::Multiply => compute_multiplication(&left_side, &right_side),
        BinaryOperationType::Divide =>  compute_division(&left_side, &right_side),
        BinaryOperationType::Modulus => compute_modulus(&left_side, &right_side),
        BinaryOperationType::And => compute_conjunction(&left_side, &right_side),
        BinaryOperationType::Or => compute_disjunction(&left_side, &right_side),
        BinaryOperationType::If => compute_implication(&left_side, &right_side),
        BinaryOperationType::Equal => todo!(),
        BinaryOperationType::NotEqual => todo!(),
        BinaryOperationType::BitwiseAnd => todo!(),
        BinaryOperationType::BitwiseOr => todo!(),
        BinaryOperationType::BitwiseXor => todo!(),
        BinaryOperationType::BitwiseLeftShift => todo!(),
        BinaryOperationType::BitwiseRightShift => todo!(),
    }
}


///
/// Computes negation of a number. Example: -2
/// 
fn compute_arithmetic_negation(operand: Value) -> Result<Value, pxpr::Error> {
    match operand.as_float() {
        Some(x) => Ok(Value::Float(-x)),
        None => Err(pxpr::Error::new(0, format!("Invalid operand for '-': {}", operand))),
    }
}


///
/// Computes the logical negation of a boolean. Example: !false
/// 
fn compute_logical_not(operand: Value) -> Result<Value, pxpr::Error> {
    match operand.as_boolean() {
        Some(b) => Ok(Value::Boolean(!b)),
        None => Err(pxpr::Error::new(0, format!("Invalid operand for '!': {}", operand))),
    }
}


fn compute_addition(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_float(), right_side.as_float()) {
        (Some(left), Some(right)) 
            => Ok(Value::Float(left + right)),

        (None, Some(_)) 
            => Err(pxpr::Error::new(0, format!("Invalid left operand for '+': {}", left_side))),
            
        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '+': {}", left_side))),
    }
}


fn compute_subtraction(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_float(), right_side.as_float()) {
        (Some(left), Some(right)) 
            => Ok(Value::Float(left - right)),

        (None, Some(_))
             => Err(pxpr::Error::new(0, format!("Invalid left operand for '-': {}", left_side))),

        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '-': {}", left_side))),
    }
}


fn compute_multiplication(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_float(), right_side.as_float()) {
        (Some(left), Some(right))
             => Ok(Value::Float(left * right)),

        (None, Some(_)) 
            => Err(pxpr::Error::new(0, format!("Invalid left operand for '*': {}", left_side))),

        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '*': {}", left_side))),
    }
}


fn compute_division(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_float(), right_side.as_float()) {
        (Some(_), Some(0.0)) 
            => Err(pxpr::Error::new(0, String::from("Division by 0"))),

        (Some(left), Some(right)) 
            => Ok(Value::Float(left / right)),

        (None, Some(_)) => 
            Err(pxpr::Error::new(0, format!("Invalid left operand for '/': {}", left_side))),

        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '/': {}", left_side))),
    }
}


fn compute_modulus(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_float(), right_side.as_float()) {
        (Some(_), Some(0.0)) 
            => Err(pxpr::Error::new(0, String::from("Division by 0"))),

        (Some(left), Some(right)) 
            => Ok(Value::Float(left % right)),

        (None, Some(_)) 
            => Err(pxpr::Error::new(0, format!("Invalid left operand for '%': {}", left_side))),
        
        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '%': {}", left_side))),
    }
}


fn compute_conjunction(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_boolean(), right_side.as_boolean()) {
        (Some(left), Some(right)) 
            => Ok(Value::Boolean(left && right)),

        (None, Some(_))
             => Err(pxpr::Error::new(0, format!("Invalid left operand for '&&': {}", left_side))),

        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '&&': {}", left_side))),
    }
}


fn compute_disjunction(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_boolean(), right_side.as_boolean()) {
        (Some(left), Some(right)) 
            => Ok(Value::Boolean(left || right)),

        (None, Some(_))
             => Err(pxpr::Error::new(0, format!("Invalid left operand for '||': {}", left_side))),

        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '||': {}", left_side))),
    }
}


fn compute_implication(left_side: &Value, right_side: &Value) -> Result<Value, pxpr::Error> {
    match (left_side.as_boolean(), right_side.as_boolean()) {
        (Some(left), Some(right)) 
            => Ok(Value::Boolean(!left || right)),

        (None, Some(_))
             => Err(pxpr::Error::new(0, format!("Invalid left operand for '=>': {}", left_side))),

        _ => Err(pxpr::Error::new(0, format!("Invalid right operand for '=>': {}", left_side))),
    }
}


