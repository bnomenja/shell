mod pwd;
mod exit;
mod cat;
mod cp;
mod rm;

use crate::parser::Command;

pub fn execute(cmd : Command) {
    match cmd.name.as_str() {
        "pwd" => pwd::run(&cmd.args),
        "exit" => exit::run(),
        "cat" => cat::run(&cmd.args),
        "cp" => cp::run(&cmd.args),
        "rm" => rm::run(&cmd.args, &cmd.options),
        unknown => println!("Command '{}' not found", unknown),
    }
}