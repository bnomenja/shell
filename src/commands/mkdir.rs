pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: missing operand");
        return;
    }

    for dir in args {
        if let Err(e) = std::fs::create_dir(dir) {
            eprintln!("Error: {}", e);
        }
    }
}