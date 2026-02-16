//! # pwd - Print Working Directory
//!
//! Displays the absolute path of the current working directory.

use std::env;

/// Executes the `pwd` command.
///
/// Displays the absolute path of the current working directory to stdout.
/// This command takes no arguments and ignores any provided.
///
/// # Arguments
/// * `_args` - Command arguments (ignored)
///
/// # Examples
/// ```
/// $ pwd
/// /Users/username/projects/0-shell
/// ```
///
/// # Errors
/// Prints an error message to stderr if the current directory cannot be determined.
pub fn execute(_args: &[String]) {
    // Get the current working directory
    match env::current_dir() {
        Ok(path) => {
            // Display the absolute path
            println!("{}", path.display());
        }
        Err(error) => {
            // Display error if current directory cannot be determined
            eprintln!("pwd: {}", error);
        }
    }
}
