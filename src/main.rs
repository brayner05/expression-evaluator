mod lexer;
mod parser;
mod expression;

use core::fmt;
use std::io::{self, Write};
use expression::{execute, Value};
use lexer::Lexer;
use parser::Parser;
use pxpr::report_error;


pub mod pxpr {
    pub struct Error {
        column: u32,
        message: String
    }

    impl Error {
        pub fn new(column: u32, message: String) -> Self {
            Error { column, message }
        }
    }

    pub fn report_error(error: &Error) {
        println!("Column {}: [ \x1b[31merror:\x1b[39m {}", &error.column, &error.message);
    }
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
fn compute_expression(raw_expression: &str) -> Result<Value, pxpr::Error> {
    let mut tokenizer = Lexer::new(raw_expression);

    // Convert the expression to a stream of tokens.
    let tokens = tokenizer.tokenize()?;

    let mut parser = Parser::new(tokens);

    // Convert the token stream to an abstract syntax tree.
    let ast = parser.parse()?;

    // Walk through the AST and compute the result.
    let result_value = execute(&ast)?;

    Ok(result_value)
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
