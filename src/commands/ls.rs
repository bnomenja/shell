use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use uzers::{get_user_by_uid, get_group_by_gid};
use chrono::{DateTime, Local, TimeZone};

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

fn print_small(names : &Vec<String>, path:&Path, show_indicator : bool) {
    names.iter().for_each(|name| {
        let full_path = path.join(name);
        
        let suffix = if show_indicator {
            indicator(&full_path)
        } else {
            ""
        };
        
        print!("{}{} ", name, suffix);
    });
}

#[derive(Debug)]
struct Entry {
    typ : char,
    permissions : String,
    nlink: u64,
    owner: String,
    group: String,
    size: String,
    mtime: i64,
    name: String,
}

fn format_mtime(mtime: i64) -> String {
    let dt: DateTime<Local> = Local.timestamp_opt(mtime, 0).unwrap();
    dt.format("%b %e %H:%M").to_string()
}

fn print_long(names : &Vec<String>, path:&Path, show_indicator : bool) {
    let mut entries : Vec<Entry> = Vec::new();
    let mut total = 0u64;

    names.iter().for_each(|name|{
        let full_path = path.join(name);
        let metadata = fs::symlink_metadata(&full_path).unwrap();

        let file_type = metadata.file_type();

        let typ = if file_type.is_dir() {
            'd'
        }else if file_type.is_block_device() {
            'b'
        }else if file_type.is_char_device(){
            'c'
        }else if file_type.is_symlink() {
            'l'
        }else if file_type.is_fifo(){
            'p'
        }else if file_type.is_socket() {
            's'
        }else {
            '-'
        };

        let uid = metadata.uid();
        let owner = match get_user_by_uid(uid) {
            Some(u) => u.name().to_string_lossy().to_string(),
            None => uid.to_string(),
        };

        let gid = metadata.gid();
        let group = match get_group_by_gid(gid) {
            Some(g) => g.name().to_string_lossy().to_string(),
            None => gid.to_string(),
        };

        let size = if typ == 'c' || typ == 'b' {
            let rdev = metadata.rdev();
            let major = libc::major(rdev);
            let minor = libc::minor(rdev);

            format!("{}, {}", major, minor)
        }else {
            metadata.size().to_string()
        };

        let formated_name : String = if file_type.is_symlink() {
            let target = match fs::read_link(&full_path) {
                Ok(t) => t.to_string_lossy().to_string(),
                Err(_) => name.to_string(),
            };

            let target_path = path.join(&target);
            let suffix = if show_indicator { 
                let indic = indicator(&target_path);
                if indic == "@" { "" }else{ indic }
             }else {
                ""
            };

            format!("{} -> {}", name, target + suffix)
        }else if show_indicator{
            name.to_string() + indicator(&full_path)
        }else {
            name.to_string()
        };

        let mtime = metadata.mtime();
        
        let permissions = format_permissions(metadata.mode());
        let nlink = metadata.nlink();

        entries.push(Entry{
            typ,
            permissions,
            nlink,
            size,
            group,
            owner,
            mtime,
            name : formated_name,
        });
        total += metadata.blocks();
    });

    total /= 2;

    let nlink_width = entries.iter().map(|e|e.nlink.to_string().len()).max().unwrap_or(0);
    let owner_width = entries.iter().map(|e|e.owner.chars().count()).max().unwrap_or(0);
    let group_width = entries.iter().map(|e|e.group.chars().count()).max().unwrap_or(0);
    let size_width = entries.iter().map(|e|e.size.to_string().len()).max().unwrap_or(0);
    let date_width = entries.iter().map(|e|e.mtime.to_string().len()).max().unwrap_or(0);

    println!("total {}", total);

    entries.iter().for_each(|e|println!(
        "{}{} {:>nlink_width$} {:<owner_width$} {:<group_width$} {:>size_width$} {:>date_width$} {}",
        e.typ, e.permissions, e.nlink, e.owner, e.group, e.size, format_mtime(e.mtime), e.name
    ));
}

pub fn format_permissions(mode :u32) -> String{
    let mut permissions : Vec<char> = Vec::new();

    let bits = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];

    for (mask, c) in bits {
        permissions.push(if mode & mask != 0 { c }else{ '-' });
    }

    //set_uid
    if mode & 0o4000 != 0 {
        permissions[2] = if mode & 0o100 != 0 { 's' }else { 'S' };
    }

    //set_gid
    if mode & 0o2000 != 0 {
        permissions[5] = if mode & 0o010 != 0 { 's' }else { 'S' };
    }

    //sticky_bit
    if mode & 0o1000 != 0 {
        permissions[8] = if mode & 0o001 != 0 { 't' }else { 'T' };
    }

    permissions.into_iter().collect()
}

pub fn run(args : &[String], options: &String) {
    let mut paths: Vec<PathBuf> = if args.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.iter().map(PathBuf::from).collect()
    };    

    paths.sort_by_key(|p| if p.is_dir(){ 1 }else{ 0 });

    let show_hidden = options.contains("a");
    let show_indicator = options.contains("F");
    let show_long = options.contains("l");
    
    for (i, path) in paths.iter().enumerate() {
        let mut names : Vec<String> = Vec::new();
        
        if !path.exists() {
            eprintln!("Error: No such file or directory '{}'", path.display());
            continue;
        }
        
        if !path.is_dir() {
            let suffix = if show_indicator {
                indicator(&path)
            } else {
                ""
            };

            print!("{}{} ", path.display(), suffix);

            // if i +1 < paths.len() && paths[i+1].is_dir(){
            //     println!();
            // }

            continue;
        } 

        if args.len() > 1 {
            println!("{}:", path.display());
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
        
        if show_long {
            print_long(&names,&path, show_indicator);
        }else{
            print_small(&names,&path, show_indicator);
        }
    }
}
