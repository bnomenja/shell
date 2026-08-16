use crate::helpers;
use std::fs::OpenOptions;
use std::time::SystemTime;

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("\x1b[31mError: missing operand\x1b[0m");
        return;
    }

    for file in args {
        let real_path = helpers::replace_tilda(file);
        
        let opened = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&real_path);

        match opened {
            Ok(f) => {
                if let Err(e) = f.set_modified(SystemTime::now()) {
                    eprintln!("\x1b[31mError: {}\x1b[0m", e);
                }
            }
            Err(e) => eprintln!("\x1b[31mError: {}\x1b[0m", e),
        }
    }
}