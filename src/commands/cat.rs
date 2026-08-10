pub fn run(args : &[String]) {
    if args.is_empty() {
        println!("\x1b[31mError: no file to display specified\x1b[0m");
        return
    }

    for path in args {
        match std::fs::read_to_string(path) {
            Ok(content) => println!("{}", content),
            Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err.kind()),
        };
    }
}