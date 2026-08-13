use std::env;
use std::path::PathBuf;
use crate::helpers;

pub fn run(args: &[String], old_pwd: &mut String) {
    let current_dir = helpers::formatted_current();
    let mut go_to_prev = false;

    let target = match args.len() {
        0 => "~".to_string(),

        1 => {
            if args[0] == "-" {
                go_to_prev = true;
                old_pwd.clone()
            } else {
                args[0].to_string()
            }
        }

        _ => {
            eprintln!("\x1b[31mError: too much arguments\x1b[0m");
            return;
        }
    };

    let real_path = helpers::replace_tilda(&target);

    let target_path = PathBuf::from(&real_path);

    if go_to_prev {
        println!("{}", old_pwd);
    }

    match env::set_current_dir(&target_path) {
        Ok(_) => *old_pwd = current_dir,
        Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err),
    }
}