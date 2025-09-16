use std::fs;
use std::path::Path;

#[derive(Default)]
struct RmFlags {
    recursive: bool,  // -r flag
}

/// Remove files and directories
/// Usage: rm [-r] <file1> [file2] ...
/// -r: Remove directories recursively
pub fn execute(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    
    let mut flags = RmFlags::default();
    let mut files = Vec::new();
    
    // Parse arguments
    for arg in args {
        if arg.starts_with('-') {
            // Parse flags
            for ch in arg.chars().skip(1) {
                match ch {
                    'r' => flags.recursive = true,
                    _ => {
                        eprintln!("rm: invalid option -- '{}'", ch);
                        return;
                    }
                }
            }
        } else {
            // File/directory argument
            files.push(*arg);
        }
    }
    
    if files.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    
    // Remove each file/directory
    for file in files {
        if let Err(e) = remove_path(file, &flags) {
            eprintln!("rm: {}: {}", file, e);
        }
    }
}

fn remove_path(path_str: &str, flags: &RmFlags) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path_str);
    
    if !path.exists() {
        return Err("No such file or directory".into());
    }
    
    if path.is_file() {
        // Remove regular file
        fs::remove_file(path)?;
    } else if path.is_dir() {
        if flags.recursive {
            // Remove directory recursively
            fs::remove_dir_all(path)?;
        } else {
            return Err("Is a directory (use -r to remove directories)".into());
        }
    } else {
        return Err("Not a regular file or directory".into());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_remove_nonexistent_file() {
        let result = remove_path("nonexistent.txt", &RmFlags::default());
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("No such file or directory"));
        }
    }
    
    #[test]
    fn test_remove_file() {
        // Create a test file
        fs::write("test_rm_file.txt", "test content").unwrap();
        
        // Remove it
        let result = remove_path("test_rm_file.txt", &RmFlags::default());
        assert!(result.is_ok());
        
        // Verify it's gone
        assert!(!Path::new("test_rm_file.txt").exists());
    }
    
    #[test]
    fn test_remove_directory_without_recursive() {
        // Create a test directory
        fs::create_dir("test_rm_dir").unwrap();
        
        // Try to remove without -r flag
        let result = remove_path("test_rm_dir", &RmFlags::default());
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Is a directory"));
        }
        
        // Clean up
        fs::remove_dir("test_rm_dir").unwrap();
    }
    
    #[test]
    fn test_remove_directory_with_recursive() {
        // Create a test directory with content
        fs::create_dir("test_rm_recursive").unwrap();
        fs::write("test_rm_recursive/file.txt", "content").unwrap();
        fs::create_dir("test_rm_recursive/subdir").unwrap();
        fs::write("test_rm_recursive/subdir/nested.txt", "nested").unwrap();
        
        // Remove recursively
        let flags = RmFlags { recursive: true };
        let result = remove_path("test_rm_recursive", &flags);
        assert!(result.is_ok());
        
        // Verify it's gone
        assert!(!Path::new("test_rm_recursive").exists());
    }
    
    #[test]
    fn test_remove_multiple_files() {
        // Create test files
        fs::write("test_rm1.txt", "content1").unwrap();
        fs::write("test_rm2.txt", "content2").unwrap();
        
        // Remove both
        let result1 = remove_path("test_rm1.txt", &RmFlags::default());
        let result2 = remove_path("test_rm2.txt", &RmFlags::default());
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        
        // Verify they're gone
        assert!(!Path::new("test_rm1.txt").exists());
        assert!(!Path::new("test_rm2.txt").exists());
    }
}
