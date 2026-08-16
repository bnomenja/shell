use std::io::{self, Write};
use std::env;
use std::path::Path;

pub fn print_propmpt() {
    const CYAN: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    let current_dir = formatted_current();

    print!("{}{}{}$ ",CYAN, current_dir, RESET);

    if let Err(e) = io::stdout().flush() {
        eprintln!("\x1b[31mError: {}\x1b[0m", e);
    }
}

pub fn formatted_current() -> String {
    let mut current_dir = match env::current_dir() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => String::new(),
    };

    if !current_dir.is_empty() {
        let home = match env::var("HOME") {
            Ok(h) => h,
            Err(_) => String::new(),
        };

        if let Some(trimed) = current_dir.strip_prefix(&home) {
            current_dir = format!("~{}", trimed);
        }
    }

    current_dir
}

pub fn replace_tilda(src : &str) -> String{
    if src.starts_with('~') {
        let home = env::var("HOME").unwrap_or_default();
        return home + src.trim_start_matches("~");
    } 
    
    src.to_string()
}

pub fn is_same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}