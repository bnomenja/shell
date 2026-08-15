use std::fs::File;
use std::io::{self, Write, Read};
use crate::helpers;

pub fn run(args : &[String]) {
    if args.is_empty() {
        loop {            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => std::process::exit(0),

                Ok(_) => {
                    print!("{}", input);
                }

                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    let mut last_byte : u8 = b'\n';

    for path in args {
        let real_path = helpers::replace_tilda(path);

        if let Err(err) = cat_file(&real_path, &mut last_byte) {
            eprintln!("\x1b[31mError: {}\x1b[0m", err);
            continue
        }
        
        if last_byte != b'\n' {
            println!();
        }

        last_byte = b'\n';
    }
}

fn cat_file(name : &str, last_byte: &mut u8) -> io::Result<()>{
    let mut file = File::open(name)?;
    let mut handler = io::stdout().lock();

    let mut buffer = [0u8; 8192];

    loop {
        let byte_read = file.read(&mut buffer)?;
        
        if byte_read == 0 {
            break;
        }

        handler.write_all(&buffer[..byte_read])?;
        *last_byte = buffer[byte_read - 1];
    }

    Ok(())
}