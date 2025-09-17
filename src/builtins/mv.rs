use std::fs;
use std::path::Path;
use std::io::ErrorKind;

/// Move/rename files and directories
/// Usage: mv <source> <destination>
/// Handles cross-filesystem moves by falling back to copy+remove
pub fn execute(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("mv: usage: mv <source> <destination>");
        return;
    }
    
    let source = args[0];
    let destination = args[1];
    
    if let Err(e) = move_file(source, destination) {
        eprintln!("mv: {}", e);
    }
}

fn move_file(source: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);
    
    // Validate source
    if !source_path.exists() {
        return Err(format!("{}: No such file or directory", source).into());
    }
    
    // Handle destination path
    let final_dest_path = if dest_path.is_dir() {
        // If destination is a directory, move file into it with same name
        let file_name = source_path.file_name()
            .ok_or("Invalid source file name")?;
        dest_path.join(file_name)
    } else {
        dest_path.to_path_buf()
    };
    
    // Check if source and destination are the same
    if source_path == final_dest_path {
        return Ok(()); // No-op, same as Unix mv behavior
    }
    
    // Try atomic rename first
    match fs::rename(source_path, &final_dest_path) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Check if this is a cross-device error
            if e.kind() == ErrorKind::CrossesDevices || 
               e.raw_os_error() == Some(18) { // EXDEV on Unix
                // Fall back to copy + remove
                copy_and_remove(source_path, &final_dest_path)
            } else {
                Err(format!("{}: {}", source, e).into())
            }
        }
    }
}

fn copy_and_remove(source: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Copy the file/directory
    if source.is_file() {
        // Copy file
        fs::copy(source, destination)?;
        
        // Preserve permissions
        let source_metadata = fs::metadata(source)?;
        let permissions = source_metadata.permissions();
        fs::set_permissions(destination, permissions)?;
        
        // Remove original file
        fs::remove_file(source)?;
    } else if source.is_dir() {
        // For directories, we'd need recursive copy+remove
        // This is complex, so for now we'll return an error
        return Err("Cross-device directory moves not supported in this minimal implementation".into());
    } else {
        return Err("Source is not a regular file or directory".into());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_move_nonexistent_file() {
        let result = move_file("nonexistent.txt", "dest.txt");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("No such file or directory"));
        }
    }
    
    #[test]
    fn test_move_file_to_file() {
        // Create a test source file
        let test_content = "Hello, move test!";
        fs::write("test_mv_source.txt", test_content).unwrap();
        
        // Move it
        let result = move_file("test_mv_source.txt", "test_mv_dest.txt");
        assert!(result.is_ok());
        
        // Verify the move
        assert!(!Path::new("test_mv_source.txt").exists());
        assert!(Path::new("test_mv_dest.txt").exists());
        
        let moved_content = fs::read_to_string("test_mv_dest.txt").unwrap();
        assert_eq!(moved_content, test_content);
        
        // Clean up
        fs::remove_file("test_mv_dest.txt").unwrap();
    }
    
    #[test]
    fn test_move_file_to_directory() {
        // Create test source file
        let test_content = "Directory move test";
        fs::write("test_mv_source_dir.txt", test_content).unwrap();
        
        // Create test directory
        fs::create_dir("test_mv_dir").unwrap();
        
        // Move file to directory
        let result = move_file("test_mv_source_dir.txt", "test_mv_dir");
        assert!(result.is_ok());
        
        // Verify the move
        assert!(!Path::new("test_mv_source_dir.txt").exists());
        let moved_path = Path::new("test_mv_dir").join("test_mv_source_dir.txt");
        assert!(moved_path.exists());
        
        let moved_content = fs::read_to_string(&moved_path).unwrap();
        assert_eq!(moved_content, test_content);
        
        // Clean up
        fs::remove_file(&moved_path).unwrap();
        fs::remove_dir("test_mv_dir").unwrap();
    }
    
    #[test]
    fn test_move_same_path() {
        // Create a test file
        let test_content = "Same path test";
        fs::write("test_mv_same.txt", test_content).unwrap();
        
        // Move to itself (should be no-op)
        let result = move_file("test_mv_same.txt", "test_mv_same.txt");
        assert!(result.is_ok());
        
        // Verify file still exists with same content
        assert!(Path::new("test_mv_same.txt").exists());
        let content = fs::read_to_string("test_mv_same.txt").unwrap();
        assert_eq!(content, test_content);
        
        // Clean up
        fs::remove_file("test_mv_same.txt").unwrap();
    }
    
    #[test]
    fn test_copy_and_remove_fallback() {
        // Create a test file
        let test_content = "Copy and remove test";
        fs::write("test_copy_remove.txt", test_content).unwrap();
        
        let source_path = Path::new("test_copy_remove.txt");
        let dest_path = Path::new("test_copy_remove_dest.txt");
        
        // Test the copy_and_remove function directly
        let result = copy_and_remove(source_path, dest_path);
        assert!(result.is_ok());
        
        // Verify the operation
        assert!(!source_path.exists());
        assert!(dest_path.exists());
        
        let moved_content = fs::read_to_string(dest_path).unwrap();
        assert_eq!(moved_content, test_content);
        
        // Clean up
        fs::remove_file(dest_path).unwrap();
    }
}
