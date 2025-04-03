use core::fmt;
use std::path::Display;

use crate::lexer::{LexerError, Token};


#[derive(Debug)]
pub enum BinaryOperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus
}

#[derive(Debug)]
pub enum AstNode {
    BinaryOperation(BinaryOperationType, Box<AstNode>, Box<AstNode>),
    UnaryOperation(BinaryOperationType, Box<AstNode>),
    Number(f64)
}


#[derive(Debug)]
pub struct ParserError {
    pub message: String
}


impl ParserError {
    fn new(message: &str) -> Self {
        ParserError { message: message.to_string() }
    }
}


pub struct Parser <'a> {
    token_stream: &'a Vec<Box<Token>>,
    current_position: usize,
}


impl <'a> Parser<'a> {
    pub fn new(token_stream: &'a Vec<Box<Token>>) -> Self {
        Parser { 
            token_stream, 
            current_position: 0, 
        }
    }


    fn has_next(&self) -> bool {
        self.current_position < self.token_stream.len()
    }


    fn peek(&self) -> Option<&Token> {
        if !self.has_next() {
            return None;
        }
        let next_token = &self.token_stream[self.current_position];
        Some(&next_token)
    }


    fn advance(&mut self) -> Result<&Token, ParserError> {
        if !self.has_next() {
            return Err(ParserError::new("Cannot read past the end of the token stream."))
        }

        let next_token = &self.token_stream[self.current_position];
        self.current_position += 1;

        Ok(next_token)
    }


    fn parse_factor(&mut self) -> Result<Box<AstNode>, ParserError> {
        let next_token = self.advance();
        if let Err(e) = next_token {
            return Err(e);
        }
        match *next_token.unwrap() {
            Token::LeftParen => self.parse_expression(),
            Token::Number(n) => Ok(Box::new(AstNode::Number(n))),
            _ => Err(ParserError::new("Expected an expression."))
        }
    }


    fn parse_term(&mut self) -> Result<Box<AstNode>, ParserError> {
        let mut left_hand = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match *token {
                Token::Asterisk => {
                    self.advance().unwrap();
                    let right_hand = self.parse_factor()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Multiply, left_hand, right_hand));
                },
                Token::Slash => {
                    self.advance().unwrap();
                    let right_hand = self.parse_factor()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Divide, left_hand, right_hand));
                },
                _ => {
                    break;
                }
            }
        }

        Ok(left_hand)
    }


    fn parse_expression(&mut self) -> Result<Box<AstNode>, ParserError> {
        let mut left_hand = self.parse_term()?;

        while let Some(token) = self.peek() {
            match *token {
                Token::Plus => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Add, left_hand, right_hand));
                },
                Token::Minus => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Subtract, left_hand, right_hand));
                },
                _ => {
                    break;
                }
            }
        }

        Ok(left_hand)
    }


    pub fn parse(&mut self) -> Result<Box<AstNode>, ParserError> {
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