pub fn run(args : &[String]) {
    if !args.is_empty() {
        println!("\x1b[31mError: too many arguments\x1b[0m");
        return;
    }

    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err),
    };

}