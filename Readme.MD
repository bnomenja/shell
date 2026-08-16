# 0-shell

A minimal Unix-like shell written in Rust, implementing a REPL with a set of
built-in commands, quote/escape-aware parsing, and multi-line input support —
no external shell (`sh`, `bash`) is spawned to run these commands.

## Features

- Interactive prompt showing the current directory (with `~` for `$HOME`)
- Custom parser supporting:
  - single and double quotes
  - backslash escaping
  - line continuation with `\` at end of line
  - `#` comments
  - `--` to mark the end of option parsing
- 12 built-in commands (see below)
- Colored error messages

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with **edition 2024**
  support (a recent stable toolchain, e.g. via `rustup`)

## Build & Run

```bash
git clone <this-repo-url>
cd 0-shell
cargo build --release
./target/release/shell
```

Or run directly in debug mode:

```bash
cargo run
```

## Usage

```
~$ help
0-shell: built-in commands

  cat     concatenate files and print on the standard output
  cd      change the current working directory
  cp      copy files to another location
  echo    display a line of text
  exit    exit the shell
  help    display help for built-in commands
  ls      list directory contents
  mkdir   create directories
  mv      move (rename) files
  pwd     print the current working directory
  rm      remove files or directories
  touch   create empty files

Type 'help <command>' for details about a specific command.
```

Type `help <command>` for the full man-page-style documentation of any
built-in (synopsis, description, options).

## Built-in commands

| Command | Synopsis                | Description                                   |
|---------|--------------------------|------------------------------------------------|
| `cat`   | `cat [FILE...]`          | print file contents, or stdin if no file given |
| `cd`    | `cd [DIRECTORY \| -]`     | change directory (`~` for home, `-` for previous) |
| `cp`    | `cp SOURCE... DEST`      | copy one or more files                         |
| `echo`  | `echo [STRING...]`       | print text (supports `\n`, `\t`, `\\`)         |
| `exit`  | `exit [CODE]`            | exit the shell with an optional status code    |
| `help`  | `help [COMMAND]`         | list commands or show detailed help            |
| `ls`    | `ls [-alF] [FILE...]`    | list directory contents                        |
| `mkdir` | `mkdir DIRECTORY...`     | create one or more directories                 |
| `mv`    | `mv SOURCE... DEST`      | move/rename one or more files                  |
| `pwd`   | `pwd`                    | print the current working directory            |
| `rm`    | `rm [-r] FILE...`        | remove files, `-r` for directories             |
| `touch` | `touch FILE...`          | create files, or update their mtime if they already exist |

### `ls` options

- `-a` — show hidden entries (`.` and files starting with `.`)
- `-l` — long listing format (permissions, links, owner, group, size, date)
- `-F` — append a type indicator (`/`, `*`, `@`, `|`, `=`)

### `rm` options

- `-r` — remove directories recursively (refused by default; `.`, `..` and
  `/` are always refused for safety)

## Project structure

```
src/
├── main.rs           # REPL loop
├── parser.rs          # tokenizer / command parser (quotes, escapes, comments)
├── helpers.rs          # shared helpers (prompt, ~ expansion, path comparison)
└── commands/
    ├── mod.rs          # command dispatch table
    ├── cat.rs
    ├── cd.rs
    ├── cp.rs
    ├── echo.rs
    ├── exit.rs
    ├── help.rs
    ├── ls.rs
    ├── mkdir.rs
    ├── mv.rs
    ├── pwd.rs
    ├── rm.rs
    └── touch.rs
```

## Dependencies

- [`jiff`](https://crates.io/crates/jiff) — date/time formatting (used by `ls -l`)
- [`libc`](https://crates.io/crates/libc) — access to `major()`/`minor()` device numbers
- [`uzers`](https://crates.io/crates/uzers) — UID/GID to username/group name resolution

## Author

**bnomenja** — [github.com/bnomenja](https://github.com/bnomenja)