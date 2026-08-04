pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("mkdir error: missing operand");
        return;
    }

    for dir in args {
        if let Err(e) = std::fs::create_dir(dir) {
            eprintln!("mkdir error for: {}, {}", dir, e);
        }
    }
}