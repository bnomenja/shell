use crate::helpers;
use std::fs;

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("\x1b[31mError: missing operand\x1b[0m");
        return;
    }

    for dir in args {
        let real_path = helpers::replace_tilda(dir);

        if let Err(e) = fs::create_dir(&real_path) {
            eprintln!("\x1b[31mError: {}\x1b[0m", e);
        }
    }
}