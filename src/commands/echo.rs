fn escape_if_needed(s: &str) -> String {
    let mut res = String::new();
    let mut chars = s.chars();
    
    while let Some(c) = chars.next() {
        if c != '\\' {
            res.push(c);
            continue;
        }

        match chars.next() {
            Some('n') => res.push('\n'),
            Some('t') => res.push('\t'),
            Some('\\') => res.push('\\'),
            Some(other) => {
                res.push('\\');
                res.push(other);
            }
            None => res.push('\\'),
        }
    }

    res
}

pub fn run(args : &[String]) {
    if args.is_empty() {
        println!();
        return;
    }

    for (i, w) in args.iter().enumerate(){
        let formated = escape_if_needed(&w);
        print!("{}", formated);
        
        if i != args.len()-1 {
            print!(" ");
        }
    }
    println!();
}