mod parser;
mod commands;

use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => std::process::exit(0),

            Ok(_) => {
                if let Some(cmd) = parser::parse(&input) {
                    commands::execute(cmd);
                }
            }

            Err(e) => eprintln!("Error: {}", e),
        }
    }
}