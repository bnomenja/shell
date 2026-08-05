mod pwd;
mod exit;
mod cat;
mod cp;
mod rm;
mod mv;
mod cd;
mod echo;
mod ls;
mod mkdir;

use crate::parser::Command;

pub fn execute(cmd : Command) {
    match cmd.name.as_str() {
        "pwd" => pwd::run(&cmd.args),
        "exit" => exit::run(),
        "cat" => cat::run(&cmd.args),
        "cp" => cp::run(&cmd.args),
        "rm" => rm::run(&cmd.args, &cmd.options),
        "mv" => mv::run(&cmd.args, &cmd.options),
        "cd" => cd::run(&cmd.args),
        "echo" => echo::run(&cmd.args),
        "mkdir" => mkdir::run(&cmd.args),
        "ls" => ls::run(&cmd.args, &cmd.options),
        unknown => println!("Command '{:?}' not found", unknown),
    }
}