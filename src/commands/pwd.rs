pub fn run(args : &[String]) {
    if !args.is_empty() {
        println!("Error: too many arguments");
        return;
    }

    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("Error: {}", err),
    };

}