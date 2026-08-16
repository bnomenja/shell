use std::fs;
use std::path::Path;
use crate::helpers;

pub fn run(args : &[String], options : &String) {
    if args.is_empty() {
        eprintln!("\x1b[31mError: missing operand\x1b[0m");
        return;
    }

    let recursive = options.contains('r');

    for target in args {
        if target == "." || target.ends_with("/.") || target == ".." || target.ends_with("/..")  || target == "/"{
            eprintln!("\x1b[31mError: for security purpose we are skipping '{}'\x1b[0m", target);
            continue;
        }

        let replaced = helpers::replace_tilda(target);
        let path = Path::new(&replaced);

        if !path.exists() {
            eprintln!("\x1b[31mError: cannot remove '{}': No such file or directory\x1b[0m", target);
            continue;
        }

        if path.is_dir() && !recursive {
            eprintln!("\x1b[31mError: cannot remove '{}': Is a directory\x1b[0m", target);
            continue;
        }

        let result = if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = result {
            eprintln!("\x1b[31mError: cannot remove '{}': {}\x1b[0m", target, e.kind());
        }
    }
}