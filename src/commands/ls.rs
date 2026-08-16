use std::path::{Path, PathBuf};
use std::io::ErrorKind;
use std::fs::{self, FileType, Metadata};
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use uzers::{get_user_by_uid, get_group_by_gid};
use jiff::{Timestamp, tz::TimeZone};
use crate::helpers;

#[derive(Debug)]
struct Entry {
    typ: char,
    permissions: String,
    nlink: u64,
    owner: String,
    group: String,
    size: String,
    mtime: i64,
    name: String,
}

pub fn run(args: &[String], options: &String) {
    let show_hidden = options.contains("a");
    let show_indicator = options.contains("F");
    let show_long = options.contains("l");

    let mut files: Vec<PathBuf> = Vec::new();
    let mut directories: Vec<PathBuf> = Vec::new();

    if args.is_empty() {
        directories.push(PathBuf::from("."));
    } else {
        for arg in args.iter() {
            let replaced = helpers::replace_tilda(arg);
            let path = PathBuf::from(&replaced);

            let sym_metadata = match fs::symlink_metadata(&path) {
                Ok(m) => m,

                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => {
                            println!("\x1b[31mError: No such file or directory '{}'\x1b[0m", path.display());
                        }

                        _ => {
                            println!("\x1b[31mError: {}\x1b[0m", e);
                        }
                    }

                    continue;
                }
            };

            if sym_metadata.file_type().is_dir() {
                directories.push(path);
            } else if sym_metadata.file_type().is_symlink() {

                if show_indicator || show_long {
                    files.push(path);
                    continue;
                }

                let metadata = match fs::metadata(&path) {
                    Ok(m) => m,

                    Err(e) => {
                        match e.kind() {
                            ErrorKind::NotFound => {
                                println!("\x1b[31mError: No such file or directory '{}'\x1b[0m", path.display());
                                //files.push(path);
                            }

                            _ => {
                                println!("\x1b[31mError: {}\x1b[0m", e);
                            }
                        }
                        continue;
                    }
                };

                if metadata.is_dir() {
                    directories.push(path);
                }else {
                    files.push(path);
                }

            }else{
                files.push(path);
            }
            
        }
    }

    files.sort_by_key(|p| sort_key(p));
    directories.sort_by_key(|p| sort_key(p));

    if show_long {
        print_long(&files, show_indicator, false);
    } else {
        print_short(&files, show_indicator);
    }

    if !files.is_empty() && !directories.is_empty() {
        println!();
    }

    for i in 0..directories.len() {
        let dir_path = &directories[i];

        if args.len() > 1 {
            println!("{}:", dir_path.display());
        }

        let mut entries: Vec<PathBuf> = Vec::new();

        if show_hidden {
            entries.push(dir_path.join("."));
            entries.push(dir_path.join(".."));
        }

        match fs::read_dir(dir_path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(data) => {
                            let entry_path = data.path();

                            let is_hidden = match entry_path.file_name() {
                                Some(name) => name.to_string_lossy().starts_with('.'),
                                None => false,
                            };

                            if is_hidden && !show_hidden {
                                continue;
                            }

                            entries.push(entry_path);
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            continue;
                        },
                    }
                }
            }
            Err(err) => {
                eprintln!("\x1b[31mError: {}\x1b[0m", err);
                continue;
            },
        }

        entries.sort_by_key(|e| sort_key(e));

        if show_long {
            print_long(&entries, show_indicator, true);
        } else {
            print_short(&entries, show_indicator);
        }

        if i + 1 < directories.len() {
            println!();
        }
    }
}

fn print_short(entries: &[PathBuf], show_indicator: bool) {
    for e in entries.iter() {
        let suffix = if show_indicator { indicator(e) } else { "" };

        let path_str = e.to_string_lossy();
        
        let name = if path_str == "." || path_str.ends_with("/.") {
            ".".to_string()
        } else if path_str == ".." || path_str.ends_with("/..") {
            "..".to_string()
        } else {
            match e.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => path_str.to_string(),
            }
        };

        let name = quote_if_needed(&name);

        print!("{}{} ", name, suffix);
    }

    if !entries.is_empty() {
        println!();
    }
}

fn print_long(entries: &[PathBuf], show_indicator: bool, show_total: bool) {
    let mut formatted_entries: Vec<Entry> = Vec::new();
    let mut total = 0u64;

    for entry_path in entries.iter() {
        let metadata = match fs::symlink_metadata(entry_path) {
            Ok(mdata) => mdata,
            Err(err ) => {
                eprintln!("\x1b[31mError for: '{}', {}\x1b[0m", entry_path.display(), err);
                continue;
            },
        };

        let typ = get_type(&metadata.file_type());

        formatted_entries.push(Entry {
            typ,
            permissions: format_permissions(metadata.mode()),
            nlink: metadata.nlink(),
            size: get_size(&typ, &metadata),
            owner: get_owner(&metadata),
            group: get_group(&metadata),
            mtime: metadata.mtime(),
            name: get_name(&typ, entry_path, show_indicator),
        });

        total += metadata.blocks();
    }

    total /= 2;

    let nlink_width = formatted_entries.iter().map(|e| e.nlink.to_string().len()).max().unwrap_or(0);
    let owner_width = formatted_entries.iter().map(|e| e.owner.chars().count()).max().unwrap_or(0);
    let group_width = formatted_entries.iter().map(|e| e.group.chars().count()).max().unwrap_or(0);
    let size_width = formatted_entries.iter().map(|e| e.size.chars().count()).max().unwrap_or(0);

    if show_total {
        println!("total {}", total);
    }

    formatted_entries.iter().for_each(|e| println!(
        "{}{} {:>nlink_width$} {:<owner_width$} {:<group_width$} {:>size_width$} {} {}",
        e.typ, e.permissions, e.nlink, e.owner, e.group, e.size, format_mtime(e.mtime), e.name
    ));
}

fn quote_if_needed(name: &str) -> String {
    if name.contains(char::is_whitespace) {
        format!("'{}'", name)
    } else {
        name.to_string()
    }
}

fn format_mtime(mtime: i64) -> String {
    let ts = match Timestamp::from_second(mtime){
        Ok(t) => t,
        Err(_) => return mtime.to_string()
    };

    let zoned = ts.to_zoned(TimeZone::system());

    zoned.strftime("%b %e %H:%M").to_string()
}

fn sort_key(path: &PathBuf) -> String {
    let path_str = path.to_string_lossy();
    
    let name = if path_str == "." || path_str.ends_with("/.") {
        ".".to_string()
    } else if path_str == ".." || path_str.ends_with("/.."){
        "..".to_string()
    } else {
        match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => path_str.to_string(),
        }
    };

    if name == "." || name == ".." {
        name
    } else {
        name.trim_start_matches('.').trim().to_lowercase()
    }
}


fn indicator(path: &PathBuf) -> &str {
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return "",
    };
    let file_type = metadata.file_type();

    if file_type.is_dir() {
        "/"
    } else if file_type.is_file() && metadata.mode() & 0o111 != 0 {
        "*"
    } else if file_type.is_symlink() {
        "@"
    } else if file_type.is_fifo() {
        "|"
    } else if file_type.is_socket() {
        "="
    } else {
        ""
    }
}

fn get_type(typ: &FileType) -> char {
    if typ.is_dir() {
        'd'
    } else if typ.is_block_device() {
        'b'
    } else if typ.is_char_device() {
        'c'
    } else if typ.is_symlink() {
        'l'
    } else if typ.is_fifo() {
        'p'
    } else if typ.is_socket() {
        's'
    } else {
        '-'
    }
}

fn get_owner(m: &Metadata) -> String {
    let uid = m.uid();

    match get_user_by_uid(uid) {
        Some(u) => u.name().to_string_lossy().to_string(),
        None => uid.to_string(),
    }
}

fn get_group(m: &Metadata) -> String {
    let gid = m.gid();

    match get_group_by_gid(gid) {
        Some(g) => g.name().to_string_lossy().to_string(),
        None => gid.to_string(),
    }
}

fn get_size(typ: &char, m: &Metadata) -> String {
    if *typ == 'c' || *typ == 'b' {
        let rdev = m.rdev();
        let major = libc::major(rdev);
        let minor = libc::minor(rdev);

        format!("{}, {}", major, minor)
    } else {
        m.size().to_string()
    }
}

fn get_name(typ: &char, path: &PathBuf, show_indicator: bool) -> String {
    let path_str = path.to_string_lossy();

    let name = if path_str == "." || path_str.ends_with("/."){
            ".".to_string()
        } else if path_str == ".." || path_str.ends_with("/.."){
            "..".to_string()
        } else {
            match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => path_str.to_string(),
            }
        };

    if *typ == 'l' {
        let target = match fs::read_link(path) {
            Ok(t) => t.to_string_lossy().to_string(),
            Err(_) => name.clone(),
        };

        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let target_path = parent.join(&target);

        let suffix = if show_indicator {
            let indic = indicator(&target_path);
            if indic == "@" { "" } else { indic }

        } else {
            ""
        };

        let quoted_name = quote_if_needed(&name);
        let quoted_target = quote_if_needed(&(target.clone() + suffix));

        format!("{} -> {}", quoted_name, quoted_target)
    } else if show_indicator {
        quote_if_needed(&(name + indicator(path)))
    } else {
        quote_if_needed(&name)
    }
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