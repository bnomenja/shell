use std::io::Write;
use std::io;
use std::env;

pub fn print_propmpt() {
    const CYAN: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    let current_dir = formatted_current();

    print!("{}{}{}$ ",CYAN, current_dir, RESET);
    io::stdout().flush().unwrap();
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