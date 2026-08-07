mod parser;
mod commands;
mod helpers; 

use std::io;

fn main() {
    loop {
        helpers::print_propmpt();
        
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