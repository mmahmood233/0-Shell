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
- `cp <source> <destination>` - Copy files (preserves permissions)
- `rm [-r] <file1> [file2] ...` - Remove files and directories
  - `-r` - Remove directories recursively
- `mv <source> <destination>` - Move/rename files (cross-filesystem support)
- `mkdir <dir1> [dir2] ...` - Create directories

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

## Project Status

✅ **COMPLETE** - All MVP built-ins implemented and tested!

**Test Coverage**: 26 tests passing
**Build Status**: Clean compilation with no warnings
**Unix Compliance**: Follows standard Unix command behavior

## Exit Conditions

- Type `exit` and press Enter
- Press Ctrl+D (EOF)
