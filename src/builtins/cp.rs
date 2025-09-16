use std::fs;
use std::path::Path;

/// Copy files from source to destination
/// Usage: cp <source> <destination>
/// Note: Directory copying is not supported in this minimal implementation
pub fn execute(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("cp: usage: cp <source> <destination>");
        return;
    }
    
    let source = args[0];
    let destination = args[1];
    
    if let Err(e) = copy_file(source, destination) {
        eprintln!("cp: {}", e);
    }
}

fn copy_file(source: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);
    
    // Validate source file
    if !source_path.exists() {
        return Err(format!("{}: No such file or directory", source).into());
    }
    
    if source_path.is_dir() {
        return Err(format!("{}: Is a directory (directory copying not supported)", source).into());
    }
    
    if !source_path.is_file() {
        return Err(format!("{}: Not a regular file", source).into());
    }
    
    // Handle destination path
    let final_dest_path = if dest_path.is_dir() {
        // If destination is a directory, copy file into it with same name
        let file_name = source_path.file_name()
            .ok_or("Invalid source file name")?;
        dest_path.join(file_name)
    } else {
        dest_path.to_path_buf()
    };
    
    // Perform the copy
    fs::copy(source_path, &final_dest_path)?;
    
    // Preserve file permissions
    let source_metadata = fs::metadata(source_path)?;
    let permissions = source_metadata.permissions();
    fs::set_permissions(&final_dest_path, permissions)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_copy_nonexistent_file() {
        let result = copy_file("nonexistent.txt", "dest.txt");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("No such file or directory"));
        }
    }
    
    #[test]
    fn test_copy_directory() {
        let result = copy_file("src", "dest");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Is a directory"));
        }
    }
    
    #[test]
    fn test_copy_file_to_file() {
        // Create a test source file
        let test_content = "Hello, copy test!";
        fs::write("test_source.txt", test_content).unwrap();
        
        // Copy it
        let result = copy_file("test_source.txt", "test_dest.txt");
        assert!(result.is_ok());
        
        // Verify the copy
        let copied_content = fs::read_to_string("test_dest.txt").unwrap();
        assert_eq!(copied_content, test_content);
        
        // Clean up
        fs::remove_file("test_source.txt").unwrap();
        fs::remove_file("test_dest.txt").unwrap();
    }
    
    #[test]
    fn test_copy_file_to_directory() {
        // Create test source file
        let test_content = "Directory copy test";
        fs::write("test_source_dir.txt", test_content).unwrap();
        
        // Create test directory
        fs::create_dir("test_dir").unwrap();
        
        // Copy file to directory
        let result = copy_file("test_source_dir.txt", "test_dir");
        assert!(result.is_ok());
        
        // Verify the copy exists in the directory
        let copied_path = Path::new("test_dir").join("test_source_dir.txt");
        assert!(copied_path.exists());
        let copied_content = fs::read_to_string(&copied_path).unwrap();
        assert_eq!(copied_content, test_content);
        
        // Clean up
        fs::remove_file("test_source_dir.txt").unwrap();
        fs::remove_file(&copied_path).unwrap();
        fs::remove_dir("test_dir").unwrap();
    }
}
