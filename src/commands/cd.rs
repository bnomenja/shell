use std::env;

pub fn run(args : &[String]) {
    if args.is_empty(){
        eprintln!("\x1b[31mError: missing destination operand\x1b[0m");
        return;
    }

    if args.len() > 1  {
        eprintln!("\x1b[31mError: too much arguments\x1b[0m");
        return;
    }

    match env::set_current_dir(&args[0]) {
        Ok(_) => {},
        Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err.kind()),
    }
}