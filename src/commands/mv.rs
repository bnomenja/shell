use crate::commands::rm;
use crate::commands::cp;

pub fn run(args : &[String], options : &String) {
    cp::run(args);
    rm::run(&args[..args.len()-1], options);    
} 