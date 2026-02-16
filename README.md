# 0-Shell

A minimalist Unix-like shell implemented in Rust for embedded Linux environments.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Tests](https://img.shields.io/badge/tests-28%2F28-success)]()
[![Rust](https://img.shields.io/badge/rust-std%20only-orange)]()

## Overview

0-Shell is a lightweight, standalone Unix shell inspired by BusyBox. It implements essential Unix commands using system calls without relying on external binaries or shells like bash/sh. Perfect for embedded systems, learning Unix internals, or understanding shell implementation.

## Features

### Core Functionality
- **10 Built-in Commands**: All essential Unix commands implemented from scratch
- **Flag Support**: `ls -l`, `ls -a`, `ls -F`, `rm -r` and combinations
- **Quote Handling**: Proper parsing of single and double-quoted strings
- **Error Handling**: Comprehensive Unix-style error messages
- **Zero Dependencies**: Uses only Rust standard library
- **EOF Support**: Graceful exit with Ctrl+D

### Built-in Commands
| Command | Description | Flags |
|---------|-------------|-------|
| `pwd` | Print working directory | - |
| `cd` | Change directory (supports `~`, `~/path`) | - |
| `ls` | List directory contents | `-l`, `-a`, `-F` |
| `cat` | Display file contents | - |
| `echo` | Print arguments | - |
| `cp` | Copy files | - |
| `mv` | Move/rename files | - |
| `rm` | Remove files/directories | `-r` |
| `mkdir` | Create directories | - |
| `exit` | Exit shell | - |

## Quick Start

### Build
```bash
cargo build --release
```

### Run
```bash
./target/release/zero-shell
```

### Test
```bash
cargo test
```

All 28 tests should pass ✅

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
0-Shell/
├── src/
│   ├── main.rs              # REPL loop and command dispatch
│   ├── parser.rs            # Command parsing with quote support
│   └── builtins/
│       ├── mod.rs           # Module exports
│       ├── pwd.rs           # Print working directory
│       ├── cd.rs            # Change directory
│       ├── echo.rs          # Echo command
│       ├── ls.rs            # List directory (with flags)
│       ├── cat.rs           # Concatenate files
│       ├── cp.rs            # Copy files
│       ├── rm.rs            # Remove files/directories
│       ├── mv.rs            # Move/rename files
│       └── mkdir.rs         # Make directories
├── Cargo.toml               # Minimal dependencies (std only)
├── README.md                # This file
├── IMPLEMENTATION_SUMMARY.md # Detailed implementation notes
└── USAGE_GUIDE.md           # Comprehensive usage documentation
```

## Requirements Met

✅ Display prompt and wait for input  
✅ Parse and execute commands  
✅ Return to prompt after execution  
✅ Handle Ctrl+D (EOF) gracefully  
✅ All 10 required commands implemented  
✅ All required flags supported (`ls -l/-a/-F`, `rm -r`)  
✅ Proper error messages ("Command '<name>' not found")  
✅ No external binaries used  
✅ Good coding practices  
✅ Comprehensive testing (28/28 tests passing)  

## Documentation

- **[IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)**: Detailed technical implementation notes
- **[USAGE_GUIDE.md](USAGE_GUIDE.md)**: Comprehensive command reference and examples

## Exit Shell

- Type `exit` and press Enter
- Press `Ctrl+D` (EOF)

## License

This project is created for educational purposes as part of a Unix system programming course.
