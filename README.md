# 0-Shell

A minimalist Unix-like shell written in Rust for embedded Linux systems.

## Features

- Interactive command prompt with REPL loop
- Built-in commands (no external binaries)
- Clean exit on `exit` command or EOF (Ctrl+D)
- Unix-like error handling

## Current Built-ins

- `exit` - Exit the shell
- `pwd` - Print working directory
- `cd [path]` - Change directory (supports `~` expansion)
- `echo [args...]` - Print arguments
- `ls [flags] [path]` - List directory contents
  - `-a` - Show hidden files (starting with `.`)
  - `-l` - Long format (permissions, links, owner, size, time)
  - `-F` - Classify files (`/` for directories, `*` for executables)
  - Flags can be combined: `-la`, `-alF`, etc.
- `cat [file1] [file2] ...` - Concatenate and display file contents

## Building and Running

```bash
# Build the project
cargo build

# Run the shell
cargo run

# Run tests
cargo test
```

## Usage Examples

```bash
$ pwd
/home/user/projects/0-shell

$ cd ~
$ pwd
/home/user

$ echo Hello World
Hello World

$ exit
```

## Project Structure

```
src/
├── main.rs           # Entry point and REPL loop
├── parser.rs         # Command parsing
└── builtins/         # Built-in command implementations
    ├── mod.rs
    ├── pwd.rs
    ├── cd.rs
    └── echo.rs
```

## Planned Built-ins

- `ls` (with `-a`, `-l`, `-F` flags)
- `cat` - Concatenate files
- `cp` - Copy files
- `rm` (with `-r` flag)
- `mv` - Move/rename files
- `mkdir` - Create directories

## Exit Conditions

- Type `exit` and press Enter
- Press Ctrl+D (EOF)
