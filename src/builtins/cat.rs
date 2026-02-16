//! # cat - Concatenate and Display Files
//!
//! Reads and displays the contents of one or more files to standard output.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Executes the `cat` command.
///
/// Reads and displays the contents of one or more files to stdout.
/// Files are processed sequentially in the order provided.
///
/// # Arguments
/// * `args` - Vector of file paths to display
///
/// # Examples
/// ```
/// $ cat file.txt
/// Contents of file.txt
///
/// $ cat file1.txt file2.txt
/// Contents of file1.txt
/// Contents of file2.txt
/// ```
///
/// # Errors
/// Prints error messages to stderr for:
/// - No files specified
/// - File does not exist
/// - Path is a directory
/// - Permission denied
/// - I/O errors while reading
pub fn execute(args: &[String]) {
    // Require at least one file argument
    if args.is_empty() {
        eprintln!("cat: no files specified");
        return;
    }
    
    // Process each file in order
    for filename in args.iter() {
        // Display the file contents, or print error if it fails
        if let Err(e) = cat_file(filename.as_str()) {
            eprintln!("cat: {}: {}", filename, e);
        }
    }
}

/// Reads and displays a single file's contents.
///
/// # Arguments
/// * `filename` - Path to the file to display
///
/// # Returns
/// * `Ok(())` - File was successfully read and displayed
/// * `Err` - File could not be read (with error description)
///
/// # Implementation
/// Uses buffered reading for efficiency with large files.
fn cat_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    
    // Validate that the file exists
    if !path.exists() {
        return Err(format!("No such file or directory").into());
    }
    
    // Ensure the path is a file, not a directory
    if !path.is_file() {
        return Err(format!("Is a directory").into());
    }
    
    // Open the file for reading
    let file = File::open(path)?;
    // Use buffered reader for efficient line-by-line reading
    let reader = BufReader::new(file);
    
    // Lock stdout for efficient writing
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    
    // Read and display each line
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                // Write line to stdout with newline
                writeln!(stdout_lock, "{}", line)?;
            }
            Err(e) => {
                // Return error if line reading fails
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
