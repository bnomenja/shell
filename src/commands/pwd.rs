pub fn run(args : &[String]) {
    if !args.is_empty() {
        println!("pwd error: too many arguments");
        return;
    }

    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd error: {}", err),
    };

}