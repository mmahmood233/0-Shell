use std::env;
use std::path::Path;

/// Change directory - handles cd, cd ~, cd <path>
pub fn execute(args: &[&str]) {
    let target_dir = if args.is_empty() {
        // cd with no args goes to home directory
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else if args[0] == "~" {
        // cd ~ goes to home directory
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else if args[0].starts_with("~/") {
        // cd ~/path expands ~ to home directory
        match env::var("HOME") {
            Ok(home) => {
                let path = &args[0][2..]; // Remove "~/"
                format!("{}/{}", home, path)
            }
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else {
        // cd <path> - use the provided path
        args[0].to_string()
    };

    let path = Path::new(&target_dir);
    
    if let Err(error) = env::set_current_dir(path) {
        eprintln!("cd: {}: {}", target_dir, error);
    }
}
