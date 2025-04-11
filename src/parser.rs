use std::rc::Rc;

use crate::{lexer::{Token, TokenType, TokenValue}, pxpr};

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinaryOperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    And,
    Or,
    If,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
}

#[derive(Debug)]
pub enum UnaryOperationType {
    ArithmeticNegate,
    LogicalNot,
    BitwiseNot
}

#[derive(Debug)]
pub enum AstNode {
    BinaryOperation(BinaryOperationType, Box<AstNode>, Box<AstNode>),
    UnaryOperation(UnaryOperationType, Box<AstNode>),
    Integer(i64),
    Float(f64),
    Boolean(bool)
}


pub struct Parser <'a> {
    token_stream: &'a Vec<Rc<Token>>,
    current_position: usize,
}


impl <'a> Parser<'a> {
    pub fn new(token_stream: &'a Vec<Rc<Token>>) -> Self {
        Parser { 
            token_stream, 
            current_position: 0, 
        }
    }


    fn has_next(&self) -> bool {
        self.current_position < self.token_stream.len()
    }


    ///
    /// Get the next token in the token stream, or `None` if no more tokens exist,
    /// without advancing in the token stream.
    /// 
    fn peek(&self) -> Option<Rc<Token>> {
        if !self.has_next() {
            return None;
        }
        let next_token = self.token_stream[self.current_position].clone();
        Some(next_token)
    }


    fn error(&self, message: String, column: u32) -> pxpr::Error {
        pxpr::Error::new(column, message)
    }


    ///
    /// Get the next token in the token stream and advance in the stream,
    /// or an error if no more tokens exist in the token stream.
    /// 
    fn advance(&mut self) -> Option<Rc<Token>> {
        let next_token = self.peek();
        self.current_position += 1;
        next_token
    }


    ///
    /// Parse an factor between parentheses.
    /// 
    fn parse_parentheses(&mut self) -> Result<Box<AstNode>, pxpr::Error> {
        let factor = self.parse_expression();
        
        if let None = self.peek() {
            return Err(self.error(
                String::from("Expected: ')'"), 0));
        }

        let tok = self.peek().unwrap();
        match tok.type_ {
            TokenType::RightParen => {
                self.advance();
                return factor;
            }
            _ => Err(self.error(
                String::from("Expected: ')', found"), 0))
        }
    }


    fn parse_unary_operation(&mut self, operator: UnaryOperationType) -> Result<Box<AstNode>, pxpr::Error> {
        let operand = self.parse_factor()?;
        Ok(Box::new(
            AstNode::UnaryOperation(
                operator, 
                operand
            )
        ))
    }


    ///
    /// Parse a factor, which is either a terminal such as a number,
    /// or in the case that the next token is a '(', a nested factor.
    /// 
    fn parse_factor(&mut self) -> Result<Box<AstNode>, pxpr::Error> {
        let next_token = self.advance();

        if let None = next_token {
            return Err(self.error(String::from("Expected an operand"), 0))
        }

        let tok = next_token.unwrap();

        match tok.type_ {
            TokenType::LeftParen => self.parse_parentheses(),

            TokenType::Minus => self.parse_unary_operation(UnaryOperationType::ArithmeticNegate),

            TokenType::Not => self.parse_unary_operation(UnaryOperationType::LogicalNot),

            TokenType::BitwiseNot => self.parse_unary_operation(UnaryOperationType::BitwiseNot),

            TokenType::Boolean => {
                if tok.value.is_none() {
                    return Err(self.error("Expected a boolean value".to_string(), tok.column))
                }

                let bool_value = tok.value
                            .as_ref()
                            .unwrap()
                            .as_boolean();

                if bool_value.is_none() {
                    return Err(self.error("Expected a boolean value".to_string(), tok.column))
                }

                Ok(Box::new(
                    AstNode::Boolean(bool_value.unwrap())
                ))
            },

            TokenType::Integer => {
                if tok.value.is_none() {
                    return Err(self.error("Expected an integer value".to_string(), tok.column))
                }

                let integer_value = tok.value
                            .as_ref()
                            .unwrap()
                            .as_integer();

                if integer_value.is_none() {
                    return Err(self.error("Expected an integer value".to_string(), tok.column))
                }

                Ok(Box::new(
                    AstNode::Integer(integer_value.unwrap())
                ))
            },

            TokenType::Float => {
                if tok.value.is_none() {
                    return Err(self.error("Expected a float value".to_string(), tok.column))
                }

                let float_value = tok.value
                            .as_ref()
                            .unwrap()
                            .as_float();

                if float_value.is_none() {
                    return Err(self.error("Expected a float value".to_string(), tok.column))
                }

                Ok(Box::new(
                    AstNode::Float(float_value.unwrap())
                ))
            }

            _ => Err(self.error(String::from("Expected an factor."), tok.column))
        }
    }


    ///
    /// Parse a term by splitting it into factors.
    /// 
    fn parse_term(&mut self) -> Result<Box<AstNode>, pxpr::Error> {
        let mut left_hand = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token.as_ref().type_ {
                TokenType::Asterisk => {
                    self.advance().unwrap();
                    let right_hand = self.parse_factor()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Multiply, left_hand, right_hand));
                },
                TokenType::Slash => {
                    self.advance().unwrap();
                    let right_hand = self.parse_factor()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Divide, left_hand, right_hand));
                },
                TokenType::Modulus => {
                    self.advance().unwrap();
                    let right_hand = self.parse_factor()?;
                    left_hand = Box::new(AstNode::BinaryOperation(BinaryOperationType::Modulus, left_hand, right_hand));
                },
                _ => {
                    break;
                }
            }
        }

        Ok(left_hand)
    }


    ///
    /// Parse an factor by splitting it into terms.
    /// 
    /// # Returns
    /// A `Result<Box<AstNode>, ParserError>` in which, on success,
    /// holds an abstract syntax tree representing the factor.
    ///
    /// TODO: Refactor the match conditions into a separate function.
    /// 
    fn parse_expression(&mut self) -> Result<Box<AstNode>, pxpr::Error> {
        let mut left_hand = self.parse_term()?;

        while let Some(token) = self.peek() {
            match token.as_ref().type_ {
                TokenType::Plus => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Add, left_hand, right_hand));
                },
                TokenType::Minus => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Subtract, left_hand, right_hand));
                },
                TokenType::And => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::And, left_hand, right_hand));
                },
                TokenType::Or => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Or, left_hand, right_hand));
                },
                TokenType::If => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::If, left_hand, right_hand));
                }
                TokenType::BitwiseAnd => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::BitwiseAnd, left_hand, right_hand)
                    )
                }
                TokenType::BitwiseOr => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::BitwiseOr, left_hand, right_hand)
                    )
                }
                TokenType::BitwiseXor => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::BitwiseXor, left_hand, right_hand)
                    )
                }
                TokenType::BitwiseLeftShift => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::BitwiseLeftShift, left_hand, right_hand)
                    )
                }
                TokenType::BitwiseRightShift => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::BitwiseRightShift, left_hand, right_hand)
                    )
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left_hand)
    }


    ///
    /// Parse an abstract syntax tree from a stream of tokens.
    /// 
    /// # Returns
    /// A `Result` encapsulating either a `Box<AstNode>` or a `ParserError`.
    pub fn parse(&mut self) -> Result<Box<AstNode>, pxpr::Error> {
        let root = self.parse_expression();
        match root {
            Ok(node) => {
                return Ok(node);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}