struct CmdInfo {
    name: &'static str,
    usage: &'static str,
    summary: &'static str,
    description: &'static str,
    options: &'static [(&'static str, &'static str)],
}

const COMMANDS: &[CmdInfo] = &[
    CmdInfo {
        name: "cat",
        usage: "cat [FILE...]",
        summary: "concatenate files and print on the standard output",
        description: "Reads each FILE in order and writes its content to standard \
output. If no FILE is given, reads from standard input until EOF (Ctrl+D).",
        options: &[],
    },
    CmdInfo {
        name: "cd",
        usage: "cd [DIRECTORY | -]",
        summary: "change the current working directory",
        description: "Changes the shell's current directory to DIRECTORY. With no \
argument, changes to the home directory (~). With '-', changes back to the \
previous directory.",
        options: &[],
    },
    CmdInfo {
        name: "cp",
        usage: "cp SOURCE... DESTINATION",
        summary: "copy files to another location",
        description: "Copies SOURCE to DESTINATION. If more than one SOURCE is \
given, or the last argument is a directory, each SOURCE is copied into that \
directory keeping its original file name.",
        options: &[],
    },
    CmdInfo {
        name: "echo",
        usage: "echo [STRING...]",
        summary: "display a line of text",
        description: "Prints its arguments separated by a single space, followed \
by a newline. Recognizes the escape sequences \\n (newline), \\t (tab) and \
\\\\ (backslash) inside arguments.",
        options: &[],
    },
    CmdInfo {
        name: "exit",
        usage: "exit [CODE]",
        summary: "exit the shell",
        description: "Terminates the shell. CODE must be a non-negative integer \
and becomes the exit status (modulo 256). With no CODE, exits with status 0.",
        options: &[],
    },
    CmdInfo {
        name: "help",
        usage: "help [COMMAND]",
        summary: "display help for built-in commands",
        description: "With no COMMAND, lists every built-in command with a short \
description. With COMMAND, shows the detailed usage for that command.",
        options: &[],
    },
    CmdInfo {
        name: "ls",
        usage: "ls [-alF] [FILE...]",
        summary: "list directory contents",
        description: "Lists information about FILEs (the current directory by \
default). Directories are listed after regular files; multiple targets are \
listed under their own header.",
        options: &[
            ("-a", "do not ignore entries starting with '.'"),
            ("-l", "use a long listing format (permissions, owner, size, date...)"),
            ("-F", "append an indicator to entries (/ for dirs, * for executables, @ for symlinks, | for FIFOs, = for sockets)"),
        ],
    },
    CmdInfo {
        name: "mkdir",
        usage: "mkdir DIRECTORY...",
        summary: "create directories",
        description: "Creates each DIRECTORY given, if it does not already exist. \
The parent directory must already exist.",
        options: &[],
    },
    CmdInfo {
        name: "mv",
        usage: "mv SOURCE... DESTINATION",
        summary: "move (rename) files",
        description: "Renames/moves SOURCE to DESTINATION. If more than one \
SOURCE is given, or the last argument is a directory, each SOURCE is moved \
into that directory keeping its original file name.",
        options: &[],
    },
    CmdInfo {
        name: "pwd",
        usage: "pwd",
        summary: "print the current working directory",
        description: "Prints the absolute path of the current working directory. \
Takes no arguments.",
        options: &[],
    },
    CmdInfo {
        name: "rm",
        usage: "rm [-r] FILE...",
        summary: "remove files or directories",
        description: "Removes each FILE given. By default, directories are \
refused; use -r to remove a directory and its contents recursively. For \
safety, '.', '..' and '/' are always refused.",
        options: &[
            ("-r", "remove directories and their contents recursively"),
        ],
    },
    CmdInfo {
        name: "touch",
        usage: "touch FILE...",
        summary: "create empty files or update their modification time",
        description: "Creates each FILE given as an empty file if it does not \
already exist. If FILE already exists, its content is left untouched and \
only its modification time is updated to now.",
        options: &[],
    },
];

pub fn run(args: &[String]) {
    if args.is_empty() {
        print_all();
        return;
    }

    for name in args {
        match COMMANDS.iter().find(|c| c.name == name.as_str()) {
            Some(c) => print_one(c),
            None => eprintln!("\x1b[31mhelp: no help topic for '{}'\x1b[0m", name),
        }
    }
}

fn print_all() {
    println!("0-shell: built-in commands\n");

    let name_width = COMMANDS.iter().map(|c| c.name.len()).max().unwrap_or(0);

    for c in COMMANDS {
        println!("  {:<name_width$}  {}", c.name, c.summary, name_width = name_width);
    }

    println!("\nType 'help <command>' for details about a specific command.");
}

fn print_one(c: &CmdInfo) {
    println!("NAME");
    println!("    {} - {}\n", c.name, c.summary);

    println!("SYNOPSIS");
    println!("    {}\n", c.usage);

    println!("DESCRIPTION");
    for line in wrap(c.description, 76) {
        println!("    {}", line);
    }

    if !c.options.is_empty() {
        println!();
        println!("OPTIONS");
        for (flag, desc) in c.options {
            println!("    {:<6} {}", flag, desc);
        }
    }

    println!();
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current.clear();
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}