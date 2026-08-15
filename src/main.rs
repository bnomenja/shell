mod parser;
mod commands;
mod helpers; 

use std::io;

fn main() {
    let mut old_pwd = helpers::formatted_current();

    loop {
        helpers::print_propmpt();
        
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => std::process::exit(0),

            Ok(_) => {
                if let Some(cmd) = parser::parse(&input) {
                    commands::execute(cmd, &mut old_pwd);
                }
            }

            Err(e) => eprintln!("\x1b[31mError: {}\x1b[0m", e),
        }
    }
}