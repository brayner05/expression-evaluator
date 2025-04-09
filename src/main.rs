mod lexer;
mod parser;
mod expression;

use core::fmt;
use std::io::{self, Write};
use expression::{execute, Value};
use lexer::Lexer;
use parser::Parser;


#[derive(Debug)]
enum ApplicationError {
    ParserError(parser::ParserError),
    LexerError(lexer::LexerError),
    ComputationError(expression::ComputationError)
}


impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::ParserError(parser_error) =>
                        write!(f, "{}", parser_error.message),
            ApplicationError::LexerError(lexer_error) => 
                        write!(f, "{}", lexer_error.message),
            ApplicationError::ComputationError(computation_error) => 
                        write!(f, "{}", computation_error)
        }
    }
}


fn report_error(error: &ApplicationError) {
    println!("[ \x1b[31mError\x1b[39m ]: {}", error);
}


///
/// Compute a raw expression and get the result of the computation
/// 
/// # Arguments
/// * `raw_expression` An immutable reference to the raw expression as a string.
/// 
/// # Return
/// A `Result<f64, ApplicationError>` in which the `Ok()` value (`f64`) is the
/// result of the computation and the error represents any error that happened during 
/// computation of the expression.
/// 
fn compute_expression(raw_expression: &str) -> Result<Value, ApplicationError> {
    let mut tokenizer = Lexer::new(raw_expression);

    // Convert the expression to a stream of tokens.
    let tokens = tokenizer.tokenize();
    if let Err(e) = tokens {
        return Err(ApplicationError::LexerError(e));
    }

    let mut parser = Parser::new(tokens.unwrap());

    // Convert the token stream to an abstract syntax tree.
    let ast = parser.parse();
    if let Err(e) = ast {
        return Err(ApplicationError::ParserError(e));
    }

    // Walk through the AST and compute the result.
    let result_value = execute(&ast.unwrap());
    if let Err(e) = result_value {
        return Err(ApplicationError::ComputationError(e))
    }
    
    Ok(result_value.unwrap())
}


///
/// Continouously reads lines from the user until the specified exit command
/// is entered. Then for every line entered, considers that line to be an expression,
/// and then computes the result_value of that expression.
/// 
fn run_repl() {
    let mut line = String::new();
    'repl: loop {
        print!("expr > ");
        io::stdout().flush().unwrap();

        // Read an expression from the user
        io::stdin()
            .read_line(&mut line)
            .unwrap();

        // If the user entered the quit command, break out of the REPL.
        if line.trim() == String::from(".quit") {
            break 'repl;
        }

        // Tokenize the input string.
        let computation_result = compute_expression(line.trim());

        match computation_result {
            Ok(result_value) => {
                println!("\t= {}", result_value)
            },
            Err(e) => {
                report_error(&e);
            },
        }

        line.clear();
    }
}


fn main() -> io::Result<()> {   
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() < 2 {
        run_repl();
        return Ok(());
    }

    let input = arguments[1..].join(" ");

    let computation_result = compute_expression(&input);
    match computation_result {
        Ok(result) => println!("\t= {}", result),
        Err(e) => report_error(&e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::compute_expression;

    #[test]
    fn computes_addition() {
        let source = "1.5 + 2.5";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Number(n) => {
                assert_eq!(n, 4.0);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }


    #[test]
    fn computes_subtraction() {
        let source = "9 - 4.5";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Number(n) => {
                assert_eq!(n, 4.5);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }


    #[test]
    fn computes_multiplication_by_zero() {
        let source = "72.592 * 0";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Number(n) => {
                assert_eq!(n, 0.0);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }


    #[test]
    fn computes_multiplication_by_one() {
        let source = "72.592 * 1";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Number(n) => {
                assert_eq!(n, 72.592);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }

    #[test]
    fn computes_multiplication() {
        let source = "6 * 2.5";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Number(n) => {
                assert_eq!(n, 15.0);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }


    #[test]
    fn computes_division_by_zero() {
        let source = "1 / 0";
        let result = compute_expression(source);
        assert!(result.is_err(), "Expected an error while dividing by 0.");
    }


    #[test]
    fn computes_modulus() {
        let source = "15 % 10";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Number(n) => {
                assert_eq!(n, 5.0);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }


    #[test]
    fn computes_and_true_false() {
        let source = "true && false";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Boolean(b) => {
                assert_eq!(b, false);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }

    #[test]
    fn computes_and_true_true() {
        let source = "true && true";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Boolean(b) => {
                assert_eq!(b, true);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }

    #[test]
    fn computes_and_false_false() {
        let source = "false && false";
        let result = compute_expression(source).expect("Failed to compute expression");
        match result {
            crate::expression::Value::Boolean(b) => {
                assert_eq!(b, false);
            },
            _ => {
                panic!("Expected a number");
            }
        }
    }
}