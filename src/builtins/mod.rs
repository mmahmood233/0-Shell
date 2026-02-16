//! # Built-in Commands Module
//!
//! Contains implementations of all built-in shell commands.
//! Each command is implemented as a separate module with an `execute` function.
//!
//! ## Available Commands
//! - `pwd` - Print working directory
//! - `cd` - Change directory
//! - `echo` - Display text
//! - `ls` - List directory contents (supports -l, -a, -F flags)
//! - `cat` - Display file contents
//! - `cp` - Copy files
//! - `rm` - Remove files/directories (supports -r flag)
//! - `mv` - Move/rename files
//! - `mkdir` - Create directories

pub mod pwd;
pub mod cd;
pub mod echo;
pub mod ls;
pub mod cat;
pub mod cp;
pub mod rm;
pub mod mv;
pub mod mkdir;
