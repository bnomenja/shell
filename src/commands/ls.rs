use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::{FileTypeExt, MetadataExt};

fn sort(paths: &mut Vec<PathBuf>) {
        paths.sort_by(|a, b| {
        let a_meta = std::fs::metadata(a);
        let b_meta = std::fs::metadata(b);

        match (a_meta, b_meta) {
            (Ok(a_m), Ok(b_m)) => {
                let a_is_dir = a_m.is_dir();
                let b_is_dir = b_m.is_dir();

                match (a_is_dir, b_is_dir) {
                    (false, true) => std::cmp::Ordering::Less,  
                    (true, false) => std::cmp::Ordering::Greater,
                    _ => a.cmp(b),
                }
            }
            _ => a.cmp(b), 
        }
    });
}

fn indicator(path : &Path) -> &str {
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return "",
    };    
    let file_type = metadata.file_type();

    if file_type.is_dir() {
        "/"
    }else if file_type.is_file() && metadata.mode() & 0o111 != 0 {
        "*"
    }else if file_type.is_symlink() {
        "@"
    }else if file_type.is_fifo(){
        "|"
    }else if file_type.is_socket() {
        "="
    }else {
        ""
    }
}

fn print_small(names : &Vec<String>, path:&Path, show_indicator : &bool) {
    names.iter().for_each(|name| {
        let full_path = path.join(name);
        
        let suffix = if *show_indicator {
            indicator(&full_path)
        } else {
            ""
        };
        
        print!("{}{} ", name, suffix);
    });
}

pub fn run(args : &[String], options: &String) {
    let mut paths: Vec<PathBuf> = if args.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.iter().map(PathBuf::from).collect()
    };    

    sort(&mut paths);

    let show_hidden = options.contains("a");
    let show_indicator = options.contains("F");
    let show_all = options.contains("l");
    let mut last_was_file = false;
    
    for (i, path) in paths.iter().enumerate() {
        let mut names : Vec<String> = Vec::new();
        
        if !path.exists() {
            eprintln!("Error: No such file or directory '{}'", path.display());
            continue;
        }

        if path.is_file() {
            let suffix = if show_indicator {
                indicator(&path)
            } else {
                ""
            };

            print!("{}{} ", path.display(), suffix);

            last_was_file = true;
            continue;
        } 

        if paths.len() > 1 {
            if last_was_file{
                println!("\n\n{}:", path.display());
                last_was_file = false;
            }else{
                println!("{}:", path.display());
            }
        }
        
        if show_hidden {
            names.push(".".to_string());
            names.push("..".to_string());
        }
        
        match fs::read_dir(path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(data) => {
                            let name = data.file_name().to_string_lossy().to_string();
                            
                            if name.starts_with(".") && !show_hidden {
                                continue;
                            }

                            names.push(name);
                        },
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    
        names.sort_by_key(|name| name.trim_start_matches('.').to_lowercase());
        
        print_small(&names,&path, &show_indicator);

        println!();
        if i != paths.len()-1 {
            println!();
        }
    }

    if last_was_file { println!() };
}

