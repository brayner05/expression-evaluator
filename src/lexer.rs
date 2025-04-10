use core::fmt;


#[derive(Debug, PartialEq)]
pub enum Token {
    // Miscellaneaous
    EOF,

    // Operations
    Plus, Minus, Asterisk, Slash, Caret,
    Modulus, Not, And, Or, If,
    Equal, NotEqual,

    // Parentheses
    LeftParen, RightParen,

    // Literals
    Number(f64), Boolean(bool)
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::EOF => write!(f, "EOF"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Caret => write!(f, "Caret"),
            Token::Modulus => write!(f, "Modulus"),
            Token::LeftParen => write!(f, "LeftParen"),
            Token::RightParen => write!(f, "RightParen"),
            Token::Number(n) => write!(f, "Number({})", n),
            Token::And => write!(f, "And"),
            Token::Or => write!(f, "Or"),
            Token::If => write!(f, "If"),
            Token::Not => write!(f, "Not"),
            Token::Equal => write!(f, "Equals"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::Boolean(n) => write!(f, "Boolean({})", n),
        }
    }
}


#[derive(Debug)]
pub struct LexerError {
    pub message: String
}


impl LexerError {
    fn new(message: String) -> Self {
        LexerError { message: message.to_string() }
    }
}


pub struct Lexer<'a> {
    source: &'a str,
    current_position: usize,
    token_start: usize,
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


    fn advance(&mut self) -> Result<char, LexerError> {
        let next_char = self.source
            .chars()
            .nth(self.current_position);

        match next_char {
            Some(n) => {
                self.current_position += 1;
                Ok(n)
            },
            None => Err(LexerError::new(
                String::from("Cannot read past the end of the source.")))
        }
    }


    fn peek(&self) -> char {
        if !self.has_next() {
            return '\0'
        }

        self.source
                .chars()
                .nth(self.current_position)
                .unwrap()
    }


    fn match_character(&self, ch: char) -> bool {
        self.peek() == ch
    }


    fn add_token(&mut self, token: Box<Token>) {
        self.token_list.push(token);
    }


    fn scan_number(&mut self) -> Result<(), LexerError> {
        while self.has_next() && self.peek().is_digit(10) {
            if let Err(e) = self.advance() {
                return Err(e)
            }
        }

        if self.peek() == '.' {
            if let Err(e) = self.advance() {
                return Err(e)
            }
            while self.has_next() && self.peek().is_digit(10) {
                if let Err(e) = self.advance() {
                    return Err(e)
                }
            }
        }

        let lexeme = &self.source[self.token_start..self.current_position];
        let value: Result<f64, std::num::ParseFloatError> = lexeme.parse();

        match value {
            Ok(n) => {
                self.add_token(Box::new(Token::Number(n)));
                Ok(())
            },
            Err(_) => Err(LexerError::new(
                String::from("Failed to parse float.")))
        }

    }


    fn scan_boolean(&mut self) -> Result<(), LexerError> {
        while self.has_next() && self.peek().is_alphabetic() {
            self.advance()?;
        }

        let lexeme = &self.source[self.token_start..self.current_position];

        match lexeme {
            "true" => {
                self.add_token(Box::new(Token::Boolean(true)));
                return Ok(());
            },
            "false" => {
                self.add_token(Box::new(Token::Boolean(false)));
                return Ok(());
            }
            _ => Err(LexerError::new(format!("Unrecognized token: {}", lexeme)))
        }
    }


    fn scan_next(&mut self) -> Result<(), LexerError> {
        let next = self.advance()?;
        match next {
            ' ' => {}
            '+' => {
                self.add_token(Box::new(Token::Plus));
            }
            '-' => {
                self.add_token(Box::new(Token::Minus));
            }
            '/' => {
                self.add_token(Box::new(Token::Slash));
            }
            '*' => {
                self.add_token(Box::new(Token::Asterisk));
            }
            '^' => {
                self.add_token(Box::new(Token::Caret));
            }
            '%' => {
                self.add_token(Box::new(Token::Modulus));
            }
            '(' => {
                self.add_token(Box::new(Token::LeftParen));
            }
            ')' => {
                self.add_token(Box::new(Token::RightParen));
            }
            '&' if self.match_character('&') => {
                self.advance()?;
                self.add_token(Box::new(Token::And));
            }
            '|' if self.match_character('|') => {
                self.advance()?;
                self.add_token(Box::new(Token::Or));
            }
            '!' if self.match_character('=') => {
                self.advance()?;
                self.add_token(Box::new(Token::NotEqual));
            }
            '!' => {
                self.add_token(Box::new(Token::Not));
            }
            '=' if self.match_character('=') => {
                self.advance()?;
                self.add_token(Box::new(Token::Equal));
            }
            '=' if self.match_character('>') => {
                self.advance()?;
                self.add_token(Box::new(Token::If));
            }
            't' | 'f' => {
                self.scan_boolean()?;
            }
            c if c.is_digit(10) => {
                self.scan_number()?
            }
            _ => {
                return Err(LexerError::new(
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
    pub fn tokenize(&mut self) -> Result<&Vec<Box<Token>>, LexerError> {
        while self.has_next() {
            // If scanning the next token produces an error,
            // return that error.
            if let Err(e) = self.scan_next() {
                return Err(e);
            }

            // Set the start of the current token to the current position.
            self.token_start = self.current_position;
        }

        // Add the EOF token.
        self.add_token(Box::new(Token::EOF));

        // No errors occurred so return a success result and
        // the list of tokens.
        Ok(&self.token_list)
    }
}


///
/// # Tests
/// 
#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn scan_next_basic_addition() {
        let mut lexer = Lexer::new("1 + 2");
        while lexer.has_next() {
            lexer.scan_next().expect("Failed to scan token");
            lexer.token_start = lexer.current_position;
        }
        let tokens = lexer.token_list;
        assert!(if let super::Token::Number(1.0) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::Plus = tokens[1].as_ref() { true } else { false });
        assert!(if let super::Token::Number(2.0) = tokens[2].as_ref() { true } else { false });
    }

    
    #[test]
    fn scan_next_basic_subtraction() {
        let mut lexer = Lexer::new("1 - 2");
        while lexer.has_next() {
            lexer.scan_next().expect("Failed to scan token");
            lexer.token_start = lexer.current_position;
        }
        let tokens = lexer.token_list;
        assert!(if let super::Token::Number(1.0) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::Minus = tokens[1].as_ref() { true } else { false });
        assert!(if let super::Token::Number(2.0) = tokens[2].as_ref() { true } else { false });
    }


    #[test]
    fn scan_booleans() {
        let mut lexer = Lexer::new("true false");

        let lexer_result = lexer.tokenize();
        assert!(!lexer_result.is_err());

        let tokens = lexer_result.unwrap();
        assert!(if let super::Token::Boolean(true) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::Boolean(false) = tokens[1].as_ref() { true } else { false });
    }


    #[test]
    fn scan_basic_and() {
        let mut lexer = Lexer::new("true && false");

        let lexer_result = lexer.tokenize();
        assert!(!lexer_result.is_err());

        let tokens = lexer_result.unwrap();
        assert!(if let super::Token::Boolean(true) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::And = tokens[1].as_ref() { true } else { false });
        assert!(if let super::Token::Boolean(false) = tokens[2].as_ref() { true } else { false });
    }


    #[test]
    fn scan_basic_or() {
        let mut lexer = Lexer::new("true || false");

        let lexer_result = lexer.tokenize();
        assert!(!lexer_result.is_err());

        let tokens = lexer_result.unwrap();
        assert!(if let super::Token::Boolean(true) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::Or = tokens[1].as_ref() { true } else { false });
        assert!(if let super::Token::Boolean(false) = tokens[2].as_ref() { true } else { false });
    }


    #[test]
    fn scan_basic_equality() {
        let mut lexer = Lexer::new("true == false");

        let lexer_result = lexer.tokenize();
        assert!(!lexer_result.is_err());

        let tokens = lexer_result.unwrap();
        assert!(if let super::Token::Boolean(true) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::Equal = tokens[1].as_ref() { true } else { false });
        assert!(if let super::Token::Boolean(false) = tokens[2].as_ref() { true } else { false });
    }


    #[test]
    fn scan_basic_not_equal() {
        let mut lexer = Lexer::new("true != false");

        let lexer_result = lexer.tokenize();
        assert!(!lexer_result.is_err());

        let tokens = lexer_result.unwrap();
        assert!(if let super::Token::Boolean(true) = tokens[0].as_ref() { true } else { false });
        assert!(if let super::Token::NotEqual = tokens[1].as_ref() { true } else { false });
        assert!(if let super::Token::Boolean(false) = tokens[2].as_ref() { true } else { false });
    }
}