mod lexer;
mod parser;

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;

///
/// Continouously reads lines from the user until the specified exit command
/// is entered. Then for every line entered, considers that line to be an expression,
/// and then computes the result of that expression.
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
        let mut tokenizer = Lexer::new(line.trim());
        let tokens = tokenizer.tokenize();

        if let Err(e) = tokens {
            eprintln!("{}", e.message);
            line.clear();
            continue;
        }

        let mut parser = Parser::new(tokens.unwrap());
        let ast = parser.parse();

        if let Err(e) = ast {
            eprintln!("{}", e.message);
            line.clear();
            continue;
        }

        println!("{:?}", ast.unwrap());

        line.clear();
    }
}


fn main() -> io::Result<()> {    
    run_repl();
    Ok(())
}
