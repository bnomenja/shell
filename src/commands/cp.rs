use std::fs::copy;
use std::path::Path;

pub fn run(args : &[String]) {

    match args.len() {
        0 => eprintln!("Error: missing file operand"),
        1 => eprintln!("Error: missing destination file operand after '{}'", args[0]),
        
        2 => {
            let src = Path::new(&args[0]);
            let dest = Path::new(&args[1]);
            let target = if dest.is_file() { dest }else { &dest.join(src.file_name().unwrap()) };

            match copy(&src, &target) {
                Ok(_) => {},
                Err(err) => eprintln!("Error: {}", err),
            }
        },

        _ => {
            let dest = Path::new(args.last().unwrap());
            if !dest.is_dir() {
                eprintln!("Error: destination must be a directory");
                return;
            }

            for src in &args[..args.len()-1] {
                let file_name = match Path::new(src).file_name() {
                    Some(name) => name,
                    None => {
                        eprintln!("Error: cannot determine filename for '{}'", src);
                        continue;
                    }
                };

                let target = dest.join(file_name);

                match copy(src, &target) {
                    Ok(_) => {},
                    Err(err) => eprintln!("Error from : {}", err),
                };
            }
        }
    }


}