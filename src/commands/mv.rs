use std::fs::rename;
use std::path::Path;
use crate::helpers;

pub fn run(args: &[String]) {
    match args.len() {
        0 => eprintln!("\x1b[31mError: missing file operand\x1b[0m"),

        1 => eprintln!( "\x1b[31mError: missing destination file operand after '{}'\x1b[0m", args[0]),

        _ => {
            let dest =  helpers::replace_tilda(&args.last().unwrap());
            let dest_path = Path::new(&dest);
        
            if args.len() > 2 && !dest_path.is_dir() {
                eprintln!("\x1b[31mError: destination must be a directory\x1b[0m");
                return;
            }

            for src in &args[..args.len() - 1] {
                let src = helpers::replace_tilda(src);
                let src_path = Path::new(&src);

                let file_name = match src_path.file_name() {
                    Some(name) => name,
                    None => {
                        eprintln!("\x1b[31mError: cannot determine filename for '{}'\x1b[0m", src);
                        continue;
                    }
                };

                let target = if dest_path.is_dir() {
                    dest_path.join(file_name)
                }else {
                    dest_path.to_path_buf()
                };

                if helpers::is_same_file(&src_path, &target) {
                    eprintln!("\x1b[31mError: '{}' and '{}' are the same file\x1b[0m", src, target.display());
                    continue;
                }

                println!("{} {}", src_path.display(), target.display());

                match rename(&src_path, &target) {
                    Ok(_) => {}
                    Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err),
                };
            }
        }
    }
}