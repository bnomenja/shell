use std::fs;

pub fn run(args : &[String], options: &String) {
    let paths = if args.is_empty() { &[".".to_string()] } else { args };
    let show_hidden = options.contains("a");
    
    for path in paths {
        if paths.len() > 1 {
            println!("{}:", path);
        }

        let mut names : Vec<String> = Vec::new();

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
        names.iter().for_each(|name|print!("{} ", name));
        println!();
    }
}