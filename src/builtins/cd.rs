//! # cd - Change Directory
//!
//! Changes the current working directory of the shell.

use std::env;
use std::path::Path;

/// Executes the `cd` command.
///
/// Changes the shell's current working directory to the specified path.
/// Supports several special cases:
/// - No arguments: Changes to home directory ($HOME)
/// - `~`: Changes to home directory
/// - `~/path`: Expands ~ to home directory and changes to path
/// - Any other path: Changes to the specified directory (absolute or relative)
///
/// # Arguments
/// * `args` - Command arguments (directory path)
///
/// # Examples
/// ```
/// $ cd              # Go to home directory
/// $ cd ~            # Go to home directory
/// $ cd ~/Desktop    # Go to Desktop in home directory
/// $ cd /tmp         # Go to /tmp
/// $ cd ..           # Go to parent directory
/// ```
///
/// # Errors
/// Prints an error message to stderr if:
/// - HOME environment variable is not set (when needed)
/// - Target directory does not exist
/// - Permission denied
pub fn execute(args: &[String]) {
    // Determine the target directory based on arguments
    let target_dir = if args.is_empty() {
        // cd with no arguments - go to home directory
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else if args[0].as_str() == "~" {
        // cd ~ - go to home directory
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else if args[0].as_str().starts_with("~/") {
        // cd ~/path - expand ~ to home directory
        match env::var("HOME") {
            Ok(home) => {
                // Remove "~/" prefix to get the relative path
                let path = &args[0].as_str()[2..];
                format!("{}/{}", home, path)
            }
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else {
        // cd <path> - use the provided path as-is (absolute or relative)
        args[0].clone()
    };

    // Convert string to Path and attempt to change directory
    let path = Path::new(&target_dir);
    
    // Try to change to the target directory
    if let Err(error) = env::set_current_dir(path) {
        eprintln!("cd: {}: {}", target_dir, error);
    }
}
