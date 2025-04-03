use std::io::{self, Write};

use lexer::Lexer;
mod lexer;


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

        match tokens {
            Ok(token_list) => {
                for token in token_list {
                    println!("{}", *token);
                }
            }
            Err(e) => {
                eprintln!("{}", e.message);
            }
        }

        // Clear the line buffer.
        line.clear();
    }
}


fn main() -> io::Result<()> {    
    run_repl();
    Ok(())
}
