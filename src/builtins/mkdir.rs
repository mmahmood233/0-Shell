//! # mkdir - Make Directories
//!
//! Creates one or more directories.

use std::fs;
use std::path::Path;

/// Executes the `mkdir` command.
///
/// Creates one or more directories. Each directory is created independently.
/// Parent directories must already exist.
///
/// # Arguments
/// * `args` - Vector of directory names to create
///
/// # Examples
/// ```
/// $ mkdir newdir
/// $ mkdir dir1 dir2 dir3
/// ```
///
/// # Errors
/// Prints error messages to stderr for:
/// - No directory names provided
/// - Directory already exists
/// - Parent directory does not exist
/// - Permission denied
///
/// # Note
/// The `-p` flag (create parent directories) is not supported in this minimal implementation.
pub fn execute(args: &[String]) {
    // Require at least one directory name
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }
    
    // Create each directory independently
    for dir_name in args.iter() {
        // Attempt to create the directory, print error if it fails
        if let Err(e) = create_directory(dir_name.as_str()) {
            eprintln!("mkdir: {}: {}", dir_name, e);
        }
    }
}

/// Creates a single directory.
///
/// # Arguments
/// * `dir_name` - Name/path of the directory to create
///
/// # Returns
/// * `Ok(())` - Directory was successfully created
/// * `Err` - Directory could not be created (with error description)
///
/// # Behavior
/// - Fails if the path already exists (file or directory)
/// - Fails if parent directory does not exist
/// - Creates directory with default permissions
fn create_directory(dir_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir_name);
    
    // Check if the path already exists
    if path.exists() {
        if path.is_dir() {
            // Path exists and is a directory
            return Err("File exists".into());
        } else {
            // Path exists but is not a directory (it's a file)
            return Err("File exists (not a directory)".into());
        }
    }
    
    // Create the directory (parent must exist)
    fs::create_dir(path)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_create_directory() {
        let dir_name = "test_mkdir_dir";
        
        // Create directory
        let result = create_directory(dir_name);
        assert!(result.is_ok());
        
        // Verify it exists and is a directory
        let path = Path::new(dir_name);
        assert!(path.exists());
        assert!(path.is_dir());
        
        // Clean up
        fs::remove_dir(dir_name).unwrap();
    }
    
    #[test]
    fn test_create_existing_directory() {
        let dir_name = "test_mkdir_existing";
        
        // Create directory first
        fs::create_dir(dir_name).unwrap();
        
        // Try to create it again
        let result = create_directory(dir_name);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("File exists"));
        }
        
        // Clean up
        fs::remove_dir(dir_name).unwrap();
    }
    
    #[test]
    fn test_create_directory_where_file_exists() {
        let file_name = "test_mkdir_file_conflict";
        
        // Create a file first
        fs::write(file_name, "content").unwrap();
        
        // Try to create directory with same name
        let result = create_directory(file_name);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("File exists"));
        }
        
        // Clean up
        fs::remove_file(file_name).unwrap();
    }
    
    #[test]
    fn test_create_multiple_directories() {
        let dir1 = "test_mkdir_multi1";
        let dir2 = "test_mkdir_multi2";
        
        // Create both directories
        let result1 = create_directory(dir1);
        let result2 = create_directory(dir2);
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        
        // Verify both exist
        assert!(Path::new(dir1).is_dir());
        assert!(Path::new(dir2).is_dir());
        
        // Clean up
        fs::remove_dir(dir1).unwrap();
        fs::remove_dir(dir2).unwrap();
    }
    
    #[test]
    fn test_create_directory_invalid_parent() {
        // Try to create directory with non-existent parent
        let result = create_directory("nonexistent_parent/new_dir");
        assert!(result.is_err());
        // The exact error message depends on the OS, but it should fail
    }
}
