use std::fs::copy;
use std::path::Path;

pub fn run(args : &[String]) {

    match args.len() {
        0 => eprintln!("cp error: missing file operand"),
        1 => eprintln!("cp: missing destination file operand after '{}'", args[0]),
        
        2 => match copy(&args[0], &args[1]) {
            Ok(_) => {},
            Err(err) => eprintln!("cp error: {}", err),
        },

        _ => {
            let dest = Path::new(args.last().unwrap());
            if !dest.is_dir() {
                eprintln!("cp error: destination must be a directory");
                return;
            }

            for src in &args[..args.len()-1] {
                let file_name = match Path::new(src).file_name() {
                    Some(name) => name,
                    None => {
                        eprintln!("cp error: cannot determine filename for '{}'", src);
                        continue;
                    }
                };

                let target = dest.join(file_name);

                match copy(src, &target) {
                    Ok(_) => {},
                    Err(err) => eprintln!("cp error from : {}, {}", src, err),
                };
            }
        }
    }


}