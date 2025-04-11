use std::fmt;

use crate::pxpr;


#[derive(Debug)]
pub enum TokenType {
    // Miscellaneaous
    EOF,

    // Operations
    Plus, Minus, Asterisk, Slash,
    Modulus, Not, And, Or, If,
    Equal, NotEqual,
    BitwiseNot, BitwiseAnd, BitwiseOr,
    BitwiseXor, BitwiseLeftShift, BitwiseRightShift,

    // Parentheses
    LeftParen, RightParen,

    // Literals
    Float, Integer, Boolean
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub enum TokenValue {
    Float(f64),
    Integer(i64),
    Boolean(bool)
}


impl TokenValue {
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TokenValue::Float(f) => Some(*f),
            _ => None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            TokenValue::Integer(i) => Some(*i),
            _ => None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            TokenValue::Boolean(b) => Some(*b),
            _ => None
        }
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub value: Option<TokenValue>,
    pub column: u32
}


impl Token {
    fn new(type_: TokenType, lexeme: String, value: Option<TokenValue>, column: u32) -> Self {
        Token { type_, lexeme, value, column }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "[ {}, \"{}\", {} ]", &self.type_, &self.lexeme, &value),
            None => write!(f, "[ {}, \"{}\" ]", &self.type_, &self.lexeme),
        }
    }
}


pub struct Lexer<'a> {
    source: &'a str,
    current_position: u32,
    token_start: u32,
    token_list: Vec<Box<Token>>
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            source,
            current_position: 0,
            token_start: 0,
            token_list: vec![]
        }
    }


    fn has_next(&self) -> bool {
        (self.current_position as usize) < self.source.len()
    }


    fn error(&self, message: String) -> pxpr::Error {
        pxpr::Error::new(self.current_position, message)
    }


    ///
    /// Consume the next character in the input string and return it.
    /// 
    /// # Returns
    /// The next character in the input string.
    /// 
    fn advance(&mut self) -> char {
        let next = self.peek().unwrap();
        self.current_position += 1;
        next
    }


    ///
    /// Get the next character in the input string but does not
    /// consume it.
    /// 
    /// # Returns
    /// The next character in the input string.
    /// 
    fn peek(&self) -> Option<char> {
        self.source
            .chars()
            .nth(self.current_position as usize)
    }


    fn match_character(&self, ch: char) -> bool {
        self.peek().is_some() && self.peek().unwrap() == ch
    }


    fn add_token(&mut self, token_type: TokenType) {
        let (start, end) = (self.token_start as usize, self.current_position as usize);
        let lexeme = self.source[start..end].to_string();
        self.token_list.push(Box::new(Token::new(token_type, lexeme, None, self.current_position)));
    }


    ///
    /// Scans a number literal.
    /// 
    fn scan_number(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_digit(10) {
                break;
            }
            self.advance();
        }

        let mut is_integer = true;

        if let Some('.') = self.peek() {
            is_integer = false;
            self.advance();
            while let Some(ch) = self.peek() {
                if !ch.is_digit(10) {
                    break;
                }
                self.advance();
            }
        }

        let (start, end) = (self.token_start as usize, self.current_position as usize);
        let lexeme = self.source[start..end].to_string();
        
        match is_integer {
            true => {
                let value: i64 = lexeme.parse().unwrap();
                self.token_list.push(Box::new(
                    Token::new(
                        TokenType::Integer, 
                        lexeme, 
                        Some(TokenValue::Integer(value)),
                        self.current_position
                    )
                ));
            },
            false => {
                let value: f64 = lexeme.parse().unwrap();
                self.token_list.push(Box::new(
                    Token::new(
                        TokenType::Float, 
                        lexeme, 
                        Some(TokenValue::Float(value)),
                        self.current_position
                    )
                ));
            },
        }
    }


    ///
    /// Scans a boolean literal from the input string.
    ///  
    fn scan_boolean(&mut self) -> Result<(), pxpr::Error> {
        while let Some(ch) = self.peek() {
            if !ch.is_alphabetic() {
                break;
            }
            self.advance();
        }

        let (start, end) = (self.token_start as usize, self.current_position as usize);
        let lexeme = self.source[start..end].to_string();

        match lexeme.as_str() {
            "true" => {
                self.token_list.push(Box::new(
                    Token::new(
                        TokenType::Boolean, 
                        lexeme, 
                        Some(TokenValue::Boolean(true)),
                        self.current_position
                    )
                ));

                return Ok(());
            },

            "false" => {
                self.token_list.push(Box::new(
                    Token::new(
                        TokenType::Boolean, 
                        lexeme, 
                        Some(TokenValue::Boolean(false)),
                        self.current_position
                    )
                ));
                
                return Ok(());
            }

            _ => Err(self.error(format!("Unrecognized token: {}", lexeme)))
        }
    }


    fn scan_next(&mut self) -> Result<(), pxpr::Error> {
        let next = self.advance();
        match next {
            ' ' => {}
            
            // ======================== //
            // = Arithmetic Operators = //
            // ======================== //

            '+' => {
                self.add_token(TokenType::Plus);
            }
            '-' => {
                self.add_token(TokenType::Minus);
            }
            '/' => {
                self.add_token(TokenType::Slash);
            }
            '*' => {
                self.add_token(TokenType::Asterisk);
            }
            '%' => {
                self.add_token(TokenType::Modulus);
            }
            '(' => {
                self.add_token(TokenType::LeftParen);
            }
            ')' => {
                self.add_token(TokenType::RightParen);
            }

            // ======================== //
            // = Boolean Operators    = //
            // ======================== //

            '&' if self.match_character('&') => {
                self.advance();
                self.add_token(TokenType::And);
            }
            '|' if self.match_character('|') => {
                self.advance();
                self.add_token(TokenType::Or);
            }
            '!' if self.match_character('=') => {
                self.advance();
                self.add_token(TokenType::NotEqual);
            }
            '!' => {
                self.add_token(TokenType::Not);
            }
            '=' if self.match_character('=') => {
                self.advance();
                self.add_token(TokenType::Equal);
            }
            '=' if self.match_character('>') => {
                self.advance();
                self.add_token(TokenType::If);
            }
            't' | 'f' => {
                self.scan_boolean()?;
            }

            // ======================== //
            // = Bitwise Operators    = //
            // ======================== //

            '~' => {
                self.add_token(TokenType::BitwiseNot);
            }

            '&' => {
                self.add_token(TokenType::BitwiseAnd);
            }

            '|' => {
                self.add_token(TokenType::BitwiseOr);
            }

            '^' => {
                self.add_token(TokenType::BitwiseXor);
            }

            '>' if self.match_character('>') => {
                self.advance();
                self.add_token(TokenType::BitwiseRightShift);
            }

            '<' if self.match_character('<') => {
                self.advance();
                self.add_token(TokenType::BitwiseLeftShift);
            }

            // ======================== //
            // = Number Literals      = //
            // ======================== //

            c if c.is_digit(10) => {
                self.scan_number()
            }


            // ========================== //
            // = Unrecognized character = //
            // ========================== //

            _ => {
                return Err(self.error(
                    format!("Unrecognized token: '{}'", next)))
            }
        };

        Ok(())
    }

    
    ///
    /// Convert an input string to a list of tokens.
    /// 
    /// # Returns
    /// A `&Vec<Box<Token>>` or rather a reference to a vector of heap-allocated
    /// tokens constructed from the input string.
    /// 
    pub fn tokenize(&mut self) -> Result<&Vec<Box<Token>>, pxpr::Error> {
        while self.has_next() {
            // If scanning the next token produces an error,
            // return that error.
            self.scan_next()?;

            // Set the start of the current token to the current position.
            self.token_start = self.current_position;
        }

        // Add the EOF token.
        self.add_token(TokenType::EOF);

        // No errors occurred so return a success result and
        // the list of tokens.
        Ok(&self.token_list)
    }
}

