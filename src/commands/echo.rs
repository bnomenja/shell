pub fn run(args : &[String]) {
    if args.is_empty() {
        println!();
        return;
    }

    for (i, w) in args.iter().enumerate(){
        print!("{}", w);
        
        if i != args.len()-1 {
            print!(" ");
        }
    }
    println!();
}