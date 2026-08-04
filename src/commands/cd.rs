use std::env;

pub fn run(args : &[String]) {
    if args.is_empty(){
        eprintln!("Error: missing destination operand");
        return;
    }

    if args.len() > 1  {
        eprintln!("Error: too much arguments");
        return;
    }

    match env::set_current_dir(&args[0]) {
        Ok(_) => {},
        Err(err) => eprintln!("Error: {}", err),
    }
}