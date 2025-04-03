use std::io::{self, Write};


fn run_repl() {
    let mut line = String::new();
    'repl: loop {
        print!("expr > ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut line)
            .unwrap();

        if line.trim() == String::from(".quit") {
            break 'repl;
        }
    }
}


fn main() -> io::Result<()> {    
    run_repl();
    Ok(())
}
