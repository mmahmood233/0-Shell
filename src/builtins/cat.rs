use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Concatenate and display file contents
/// Usage: cat [file1] [file2] ...
/// If no files specified, reads from stdin (not implemented yet)
pub fn execute(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: no files specified");
        return;
    }
    
    for filename in args {
        if let Err(e) = cat_file(filename) {
            eprintln!("cat: {}: {}", filename, e);
        }
    }
}

fn cat_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    
    // Check if file exists and is readable
    if !path.exists() {
        return Err(format!("No such file or directory").into());
    }
    
    if !path.is_file() {
        return Err(format!("Is a directory").into());
    }
    
    // Open and read the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // Stream the file contents to stdout
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                writeln!(stdout_lock, "{}", line)?;
            }
            Err(e) => {
                return Err(format!("Error reading file: {}", e).into());
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_cat_nonexistent_file() {
        // This test verifies error handling for non-existent files
        let result = cat_file("nonexistent_file.txt");
        assert!(result.is_err());
    }
    
    #[test] 
    fn test_cat_directory() {
        // Test that cat properly handles directories
        let result = cat_file("src");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Is a directory"));
        }
    }
    
    #[test]
    fn test_cat_existing_file() {
        // Create a temporary test file
        let test_content = "Hello, World!\nThis is a test file.\n";
        fs::write("test_cat.txt", test_content).unwrap();
        
        // Test that cat can read the file (we can't easily test stdout in unit tests)
        let result = cat_file("test_cat.txt");
        assert!(result.is_ok());
        
        // Clean up
        fs::remove_file("test_cat.txt").unwrap();
    }
}
