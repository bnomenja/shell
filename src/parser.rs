#[derive(Debug)]
pub struct Command {
    pub name : String,
    pub options : String,
    pub args : Vec<String>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            name : String::new(),
            options : String::new(),
            args: Vec::new(),
        }
    }
}

use std::io;
use std::io::Write;

fn tokenize(input: &str, cmd: &mut Command, buffer: &mut String, in_sgl_quotes: &mut bool, in_dbl_quotes: &mut bool) {
    let mut escape = false;

    for c in input.chars() {
        if escape {
            buffer.push(c);
            escape = false;
            continue;
        }

        match c {
            '\\' if !*in_sgl_quotes => escape = true,
            
            '\'' if !*in_dbl_quotes => *in_sgl_quotes = !*in_sgl_quotes,

            '"' if !*in_sgl_quotes => *in_dbl_quotes = !*in_dbl_quotes,

            '#' if buffer.is_empty() && !(*in_dbl_quotes || *in_sgl_quotes) => {
                buffer.clear();
                return;
            },

            c if c.is_whitespace() && !*in_sgl_quotes && !*in_dbl_quotes && !buffer.trim().is_empty() => {
                let trimed = buffer.trim().to_string();

                if cmd.name.is_empty() {
                    cmd.name = trimed.clone();
                } else if trimed.starts_with('-') {
                    cmd.options.push_str(&trimed[1..]);
                }else{
                    cmd.args.push(trimed.clone());
                }

                buffer.clear();
            }
            
            c => buffer.push(c),
        }
    }
}

pub fn parse(input: &str) -> Option<Command> {
    if input.trim().is_empty() {
        return None;
    }
    
    let mut cmd = Command::new();
    let mut in_sgl_quotes = false;
    let mut in_dbl_quotes = false;
    let mut buffer = String::new();
    let mut jump_line =  input.trim_end().ends_with('\\');

    let line = if jump_line { input.trim_end().trim_end_matches('\\') } else { input };    

    tokenize(&line, &mut cmd, &mut buffer, &mut in_sgl_quotes, &mut in_dbl_quotes);

    while in_sgl_quotes || in_dbl_quotes || jump_line{

        if jump_line { 
            print!("> ");
        }else if in_dbl_quotes { 
            print!("dquote> ");
        } else {  
            print!("quote> ");
        }

        io::stdout().flush().unwrap();

        let mut new_input = String::new();

        match io::stdin().read_line(&mut new_input) {
            Ok(_) => {
                jump_line = new_input.trim_end().ends_with('\\');
                let line = if jump_line { new_input.trim_end().trim_end_matches('\\') } else { new_input.as_str() };

                tokenize(line, &mut cmd, &mut buffer, &mut in_sgl_quotes, &mut in_dbl_quotes);
            }

            Err(e) => {
                eprintln!("Error: {}", e);
                return None;
            }
        }
    }

    buffer = buffer.trim().to_string();
    if !buffer.is_empty() {
        if cmd.name.is_empty() {
            cmd.name = buffer.clone();
        } else if buffer.starts_with('-') {
            cmd.options.push_str(&buffer[1..]);
        }else{
            cmd.args.push(buffer.clone());
        }
    }

    Some(cmd)
}
//echo hello "hello" 'hello' "hello'hello" 'hello"hello' "hello sakjsnas"hello