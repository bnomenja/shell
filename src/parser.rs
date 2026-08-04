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

pub fn parse(input : &str) -> Option<Command> {
    if input.is_empty(){
        return None
    }

    let mut cmd = Command::new();

    for (i, w) in input.split_whitespace().enumerate() {
        if i == 0 {
            cmd.name = w.to_string();
            continue;
        }

        if w.starts_with("-") {
            cmd.options.push_str(&w[1..]);   
        }else {
            cmd.args.push(w.to_string());
        }

    }   

    Some(cmd)
}