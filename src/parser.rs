use crate::lexer::Token;

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
}

#[derive(Debug)]
pub enum UnaryOperationType {
    ArithmeticNegate,
    LogicalNegate
}

#[derive(Debug)]
pub enum AstNode {
    BinaryOperation(BinaryOperationType, Box<AstNode>, Box<AstNode>),
    UnaryOperation(UnaryOperationType, Box<AstNode>),
    Number(f64),
    Boolean(bool)
}


#[derive(Debug)]
pub struct ParserError {
    pub message: String
}


impl ParserError {
    fn new(message: String) -> Self {
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


    ///
    /// Get the next token in the token stream, or `None` if no more tokens exist,
    /// without advancing in the token stream.
    /// 
    fn peek(&self) -> Option<&Token> {
        if !self.has_next() {
            return None;
        }
        let next_token = &self.token_stream[self.current_position];
        Some(&next_token)
    }


    ///
    /// Get the next token in the token stream and advance in the stream,
    /// or an error if no more tokens exist in the token stream.
    /// 
    fn advance(&mut self) -> Result<&Token, ParserError> {
        if !self.has_next() {
            return Err(ParserError::new(
                String::from("Cannot read past the end of the token stream.")))
        }

        let next_token = &self.token_stream[self.current_position];
        self.current_position += 1;

        Ok(next_token)
    }


    ///
    /// Parse an factor between parentheses.
    /// 
    fn parse_parentheses(&mut self) -> Result<Box<AstNode>, ParserError> {
        let factor = self.parse_expression();
        
        if let None = self.peek() {
            return Err(ParserError::new(
                String::from("Expected: ')'")));
        }

        match *self.peek().unwrap() {
            Token::RightParen => {
                self.advance()?;
                return factor;
            }
            _ => Err(ParserError::new(
                String::from("Expected: ')', found")))
        }
    }


    fn parse_negation(&mut self, operator: UnaryOperationType) -> Result<Box<AstNode>, ParserError> {
        let operand = self.parse_factor();
        if let None = self.peek() {
            return Err(ParserError::new(
                String::from("Expected a factor")));
        }
        Ok(Box::new(
            AstNode::UnaryOperation(
                operator, 
                operand.unwrap()
            )
        ))
    }


    ///
    /// Parse a factor, which is either a terminal such as a number,
    /// or in the case that the next token is a '(', a nested factor.
    /// 
    fn parse_factor(&mut self) -> Result<Box<AstNode>, ParserError> {
        let next_token = self.advance();
        if let Err(e) = next_token {
            return Err(e);
        }
        match *next_token.unwrap() {
            Token::LeftParen => self.parse_parentheses(),
            Token::Number(n) => Ok(Box::new(AstNode::Number(n))),
            Token::Boolean(b) => Ok(Box::new(AstNode::Boolean(b))),
            Token::Minus => self.parse_negation(UnaryOperationType::ArithmeticNegate),
            Token::Not => self.parse_negation(UnaryOperationType::LogicalNegate),
            _ => Err(ParserError::new(
                String::from("Expected an factor.")))
        }
    }


    ///
    /// Parse a term by splitting it into factors.
    /// 
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
                Token::Modulus => {
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
                Token::And => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::And, left_hand, right_hand));
                },
                Token::Or => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::Or, left_hand, right_hand));
                },
                Token::If => {
                    self.advance().unwrap();
                    let right_hand = self.parse_term()?;
                    left_hand = Box::new(
                        AstNode::BinaryOperation(BinaryOperationType::If, left_hand, right_hand));
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