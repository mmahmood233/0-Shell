use std::io::{self, Write};

mod parser;
mod builtins;

use parser::parse_command;
use builtins::*;

fn main() {
    println!("0-Shell v0.1.0 - Minimalist Unix-like shell");
    
    loop {
        // Display prompt
        print!("$ ");
        io::stdout().flush().unwrap();
        
        // Read input line
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D) - exit gracefully
                println!();
                break;
            }
            Ok(_) => {
                let input = input.trim();
                
                // Skip empty lines
                if input.is_empty() {
                    continue;
                }
                
                // Parse and execute command
                execute_command(input);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}

fn execute_command(input: &str) {
    let (command, args) = parse_command(input);
    
    match command {
        "exit" => {
            std::process::exit(0);
        }
        "pwd" => {
            pwd::execute(&args);
        }
        "cd" => {
            cd::execute(&args);
        }
        "echo" => {
            echo::execute(&args);
        }
        _ => {
            println!("Command '{}' not found", command);
        }
    }
}
