pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("\x1b[31mError: missing operand\x1b[0m");
        return;
    }

    for dir in args {
        if let Err(e) = std::fs::create_dir(dir) {
            eprintln!("\x1b[31mError: {}\x1b[0m", e.kind());
        }
    }
}