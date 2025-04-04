use crate::parser::{AstNode, BinaryOperationType, UnaryOperationType};

pub fn execute(expression: &Box<AstNode>) -> f64 {
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

        AstNode::Number(x) => *x,
    }
}


fn compute_unary(operation_type: &UnaryOperationType, operand: &Box<AstNode>) -> f64 {
    match operation_type {
        UnaryOperationType::Negate => -1.0 * execute(operand),
    }
}


fn compute_binary(operation_type: &BinaryOperationType, left: &Box<AstNode>, right: &Box<AstNode>) -> f64 {
    let left_side = execute(left);
    let right_side = execute(right);

    match operation_type {
        BinaryOperationType::Add => left_side + right_side,
        BinaryOperationType::Subtract => left_side - right_side,
        BinaryOperationType::Multiply => left_side * right_side,
        BinaryOperationType::Divide => left_side / right_side,
        BinaryOperationType::Modulus => left_side % right_side,
    }
}