use std::env;

/// Print working directory - displays the current directory path
pub fn execute(_args: &[&str]) {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(error) => {
            eprintln!("pwd: {}", error);
        }
    }
}
