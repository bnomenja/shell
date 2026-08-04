use std::fs;
use std::path::Path;

pub fn run(cmd: &Command) {
    if cmd.args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    let recursive = cmd.option.contains('r');

    for target in &cmd.args {
        let path = Path::new(target);

        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", target);
            continue;
        }

        if path.is_dir() && !recursive {
            eprintln!("rm: cannot remove '{}': Is a directory", target);
            continue;
        }

        let result = if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = result {
            eprintln!("rm: cannot remove '{}': {}", target, e);
        }
    }
}