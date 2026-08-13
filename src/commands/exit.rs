pub fn run(args : &[String]) {
    match args.len() {
        0 => std::process::exit(0),

        1 => {
            let num : i64 = match args[0].parse() {
                Ok(n) => {
                    if n < 0 {
                        eprintln!("\x1b[31mError: illegal number: {}\x1b[0m", n);
                        return;
                    }
                    
                    n
                },


                Err(_) => {
                    eprintln!("\x1b[31mError: invalid argument\x1b[0m");
                    return
                },
            };

            let code = (num % 256) as i32;

            std::process::exit(code);
        },

        _ => {
            eprintln!("\x1b[31mError: too many arguments\x1b[0m");
        }
    }

}