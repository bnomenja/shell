use std::fs;
use std::path::Path;

pub fn run(args : &[String], options : &String) {
    if args.is_empty() {
        eprintln!("Error: missing operand");
        return;
    }

    let recursive = options.contains('r');

    for target in args {
        if target == "." || target == ".." {
            eprintln!("Error: for security purpose we are skipping '{}'", target);
            continue;
        }

        let path = Path::new(target);

        if !path.exists() {
            eprintln!("Error: cannot remove '{}': No such file or directory", target);
            continue;
        }

        if path.is_dir() && !recursive {
            eprintln!("Error: cannot remove '{}': Is a directory", target);
            continue;
        }

        let result = if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = result {
            eprintln!("Error: cannot remove '{}': {}", target, e);
        }
    }
}