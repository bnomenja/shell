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
        "cat" => cat::run(&cmd.args),
        "cd" => cd::run(&cmd.args),
        "cp" => cp::run(&cmd.args),
        "echo" => echo::run(&cmd.args),
        "exit" => exit::run(),
        "ls" => ls::run(&cmd.args, &cmd.options),
        "mkdir" => mkdir::run(&cmd.args),
        "mv" => mv::run(&cmd.args, &cmd.options),
        "pwd" => pwd::run(&cmd.args),
        "rm" => rm::run(&cmd.args, &cmd.options),
        unknown => println!("command not found: {:?}", unknown),
    }
}
