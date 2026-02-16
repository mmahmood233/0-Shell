//! # mv - Move/Rename Files and Directories
//!
//! Moves or renames files and directories, with cross-filesystem support.

use std::fs;
use std::path::Path;
use std::io::ErrorKind;

/// Executes the `mv` command.
///
/// Moves or renames a file or directory from source to destination.
/// If destination is a directory, moves source into it with the same name.
///
/// # Arguments
/// * `args` - Command arguments: [source, destination]
///
/// # Examples
/// ```
/// $ mv oldname.txt newname.txt    # Rename file
/// $ mv file.txt mydir/            # Move into directory
/// $ mv dir1 dir2                  # Rename directory
/// ```
///
/// # Errors
/// Prints error messages to stderr for:
/// - Wrong number of arguments
/// - Source does not exist
/// - Permission denied
/// - Cross-device directory moves (not supported)
///
/// # Implementation Details
/// - Uses atomic `rename()` when source and destination are on same filesystem
/// - Falls back to copy+remove for cross-filesystem file moves
/// - Preserves file permissions during cross-filesystem moves
/// - Cross-filesystem directory moves are not supported
pub fn execute(args: &[String]) {
    // Require exactly two arguments: source and destination
    if args.len() != 2 {
        eprintln!("mv: usage: mv <source> <destination>");
        return;
    }
    
    // Extract source and destination paths
    let source = args[0].as_str();
    let destination = args[1].as_str();
    
    // Perform the move/rename operation
    if let Err(e) = move_file(source, destination) {
        eprintln!("mv: {}", e);
    }
}

/// Moves or renames a file or directory.
///
/// # Arguments
/// * `source` - Path to the source file/directory
/// * `destination` - Path to the destination (file/directory)
///
/// # Returns
/// * `Ok(())` - Move/rename was successful
/// * `Err` - Operation failed (with error description)
///
/// # Strategy
/// 1. Try atomic rename (fast, works on same filesystem)
/// 2. If cross-device error, fall back to copy+remove (files only)
/// 3. Preserve permissions during copy+remove
fn move_file(source: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);
    
    // Validate that source exists
    if !source_path.exists() {
        return Err(format!("{}: No such file or directory", source).into());
    }
    
    // Determine the final destination path
    let final_dest_path = if dest_path.is_dir() {
        // Destination is a directory - move source into it with same name
        let file_name = source_path.file_name()
            .ok_or("Invalid source file name")?;
        dest_path.join(file_name)
    } else {
        // Destination is a file path - use it as-is
        dest_path.to_path_buf()
    };
    
    // Check if source and destination are the same (no-op)
    if source_path == final_dest_path {
        // Source and destination are identical - nothing to do
        return Ok(()); // No-op, same as Unix mv behavior
    }
    
    // Attempt atomic rename (fast, works on same filesystem)
    match fs::rename(source_path, &final_dest_path) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Check if this is a cross-device/cross-filesystem error
            if e.kind() == ErrorKind::CrossesDevices || 
               e.raw_os_error() == Some(18) { // EXDEV error code on Unix
                // Rename failed due to cross-filesystem move
                // Fall back to copy+remove strategy
                copy_and_remove(source_path, &final_dest_path)
            } else {
                // Other error (permission denied, etc.)
                Err(format!("{}: {}", source, e).into())
            }
        }
    }
}

/// Fallback strategy for cross-filesystem moves.
///
/// # Arguments
/// * `source` - Source path
/// * `destination` - Destination path
///
/// # Returns
/// * `Ok(())` - Copy+remove was successful
/// * `Err` - Operation failed
///
/// # Implementation
/// 1. Copy source to destination
/// 2. Preserve permissions
/// 3. Remove original source
///
/// # Limitations
/// Only supports files. Cross-filesystem directory moves are not supported.
fn copy_and_remove(source: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Handle based on source type
    if source.is_file() {
        // Copy file contents to destination
        fs::copy(source, destination)?;
        
        // Preserve file permissions from source
        let source_metadata = fs::metadata(source)?;
        let permissions = source_metadata.permissions();
        fs::set_permissions(destination, permissions)?;
        
        // Remove the original source file
        fs::remove_file(source)?;
    } else if source.is_dir() {
        // Directory moves across filesystems require recursive copy
        // This is complex and not supported in this minimal implementation
        return Err("Cross-device directory moves not supported in this minimal implementation".into());
    } else {
        // Source is neither file nor directory (e.g., symlink)
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
