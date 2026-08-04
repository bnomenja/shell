pub fn run(args : &[String]) {
    if args.is_empty() {
        println!("cat error: no file to display specified");
        return
    }

    
    for path in args {
        match std::fs::read_to_string(path) {
            Ok(content) => println!("{}", content),
            Err(err) => eprintln!("cat error for: {}, {}", path, err),
        };
    }
}