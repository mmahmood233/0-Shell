//! # 0-Shell - Minimalist Unix-like Shell
//!
//! A lightweight shell implementation in Rust that provides core Unix commands
//! without relying on external binaries. All commands are implemented as built-ins
//! using Rust's standard library.
//!
//! ## Features
//! - REPL (Read-Eval-Print Loop) for interactive command execution
//! - 10 built-in commands: echo, cd, ls, pwd, cat, cp, rm, mv, mkdir, exit
//! - Quote-aware command parsing (supports both single and double quotes)
//! - Graceful EOF handling (Ctrl+D)
//! - Unix-like error messages

use std::io::{self, Write};

// Module declarations
mod parser;
mod builtins;

// Import the command parser
use parser::parse_command;
// Import all built-in command modules
use builtins::*;

/// Main entry point for the 0-Shell application.
///
/// Initializes the shell and enters the REPL (Read-Eval-Print Loop).
/// The loop continues until the user exits via the `exit` command or EOF (Ctrl+D).
///
/// # REPL Flow
/// 1. Display prompt (`$ `)
/// 2. Read user input from stdin
/// 3. Parse the input into command and arguments
/// 4. Execute the command
/// 5. Return to step 1
///
/// # Exit Conditions
/// - User types `exit` command
/// - User presses Ctrl+D (EOF signal)
/// - Fatal error reading from stdin
fn main() {
    // Display welcome message
    println!("0-Shell v0.1.0 - Minimalist Unix-like shell");
    
    // Main REPL loop
    loop {
        // Display the shell prompt
        print!("$ ");
        // Flush stdout to ensure prompt is displayed immediately
        io::stdout().flush().unwrap();
        
        // Read a line of input from the user
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D) received - exit gracefully
                println!();
                break;
            }
            Ok(_) => {
                // Successfully read input - trim whitespace
                let input = input.trim();
                
                // Skip empty lines (user just pressed Enter)
                if input.is_empty() {
                    continue;
                }
                
                // Parse and execute the command
                execute_command(input);
            }
            Err(error) => {
                // Error reading from stdin - display error and continue
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}

/// Parses and executes a shell command.
///
/// Takes a raw input string, parses it into a command name and arguments,
/// then dispatches to the appropriate built-in command handler.
///
/// # Arguments
/// * `input` - The raw command string entered by the user
///
/// # Supported Commands
/// - `exit` - Exit the shell (terminates the process)
/// - `pwd` - Print working directory
/// - `cd` - Change directory
/// - `echo` - Print arguments to stdout
/// - `ls` - List directory contents (supports -l, -a, -F flags)
/// - `cat` - Display file contents
/// - `cp` - Copy files
/// - `rm` - Remove files/directories (supports -r flag)
/// - `mv` - Move/rename files
/// - `mkdir` - Create directories
///
/// # Error Handling
/// If an unrecognized command is entered, displays:
/// "Command '<name>' not found"
fn execute_command(input: &str) {
    // Parse the input into command name and arguments
    let (command, args) = parse_command(input);
    
    // Dispatch to the appropriate command handler
    match command {
        "exit" => {
            // Exit command - terminate the shell process with success code
            std::process::exit(0);
        }
        "pwd" => {
            // Print working directory
            pwd::execute(&args);
        }
        "cd" => {
            // Change directory
            cd::execute(&args);
        }
        "echo" => {
            // Echo arguments to stdout
            echo::execute(&args);
        }
        "ls" => {
            // List directory contents
            ls::execute(&args);
        }
        "cat" => {
            // Concatenate and display files
            cat::execute(&args);
        }
        "cp" => {
            // Copy files
            cp::execute(&args);
        }
        "rm" => {
            // Remove files/directories
            rm::execute(&args);
        }
        "mv" => {
            // Move/rename files
            mv::execute(&args);
        }
        "mkdir" => {
            // Create directories
            mkdir::execute(&args);
        }
        _ => {
            // Unknown command - display error message
            println!("Command '{}' not found", command);
        }
    }
}
