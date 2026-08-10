use std::fs::copy;
use std::path::Path;

pub fn run(args : &[String]) {

    match args.len() {
        0 => eprintln!("\x1b[31mError: missing file operand\x1b[0m"),
        1 => eprintln!("\x1b[31mError: missing destination file operand after '{}'\x1b[0m", args[0]),
        
        2 => {
            let src = Path::new(&args[0]);
            let dest = Path::new(&args[1]);
            let target = if dest.is_file() { dest }else { &dest.join(src.file_name().unwrap()) };

            match copy(&src, &target) {
                Ok(_) => {},
                Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err.kind()),
            }
        },

        _ => {
            let dest = Path::new(args.last().unwrap());
            if !dest.is_dir() {
                eprintln!("\x1b[31mError: destination must be a directory\x1b[0m");
                return;
            }

            for src in &args[..args.len()-1] {
                let file_name = match Path::new(src).file_name() {
                    Some(name) => name,
                    None => {
                        eprintln!("\x1b[31mError: cannot determine filename for '{}'\x1b[0m", src);
                        continue;
                    }
                };

                let target = dest.join(file_name);

                match copy(src, &target) {
                    Ok(_) => {},
                    Err(err) => eprintln!("\x1b[31mError from : {}\x1b[0m", err.kind()),
                };
            }
        }
    }


}