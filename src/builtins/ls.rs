use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Default)]
struct LsFlags {
    show_all: bool,      // -a flag
    long_format: bool,   // -l flag  
    classify: bool,      // -F flag
}

/// List directory contents with support for -a, -l, -F flags
pub fn execute(args: &[&str]) {
    let mut flags = LsFlags::default();
    let mut path = ".";  // Default to current directory
    
    // Parse arguments
    for arg in args {
        if arg.starts_with('-') {
            // Parse flags
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => flags.show_all = true,
                    'l' => flags.long_format = true,
                    'F' => flags.classify = true,
                    _ => {
                        eprintln!("ls: invalid option -- '{}'", ch);
                        return;
                    }
                }
            }
        } else {
            // Path argument
            path = arg;
        }
    }
    
    // List directory contents
    if let Err(e) = list_directory(path, &flags) {
        eprintln!("ls: {}: {}", path, e);
    }
}

fn list_directory(path: &str, flags: &LsFlags) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = Path::new(path);
    let entries = fs::read_dir(dir_path)?;
    
    let mut items = Vec::new();
    
    // Collect and sort entries
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        
        // Skip hidden files unless -a flag is set
        if !flags.show_all && name.starts_with('.') {
            continue;
        }
        
        let metadata = entry.metadata()?;
        items.push((name.to_string(), entry.path(), metadata));
    }
    
    // Sort by name
    items.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Display entries
    for (name, path, metadata) in items {
        if flags.long_format {
            print_long_format(&name, &path, &metadata)?;
        } else {
            let mut display_name = name;
            
            // Add classifier suffix if -F flag is set
            if flags.classify {
                if metadata.is_dir() {
                    display_name.push('/');
                } else if is_executable(&metadata) {
                    display_name.push('*');
                }
            }
            
            println!("{}", display_name);
        }
    }
    
    Ok(())
}

fn print_long_format(name: &str, _path: &Path, metadata: &fs::Metadata) -> Result<(), Box<dyn std::error::Error>> {
    // Format: mode links uid:gid size mtime name
    let mode_str = format_mode(metadata.mode());
    let nlink = metadata.nlink();
    let uid = metadata.uid();
    let gid = metadata.gid();
    let size = metadata.size();
    let mtime = format_time(metadata.mtime())?;
    
    let mut display_name = name.to_string();
    
    // Add classifier for directories and executables
    if metadata.is_dir() {
        display_name.push('/');
    } else if is_executable(metadata) {
        display_name.push('*');
    }
    
    println!("{} {:3} {}:{} {:8} {} {}", 
             mode_str, nlink, uid, gid, size, mtime, display_name);
    
    Ok(())
}

fn format_mode(mode: u32) -> String {
    let mut result = String::with_capacity(10);
    
    // File type
    result.push(match mode & 0o170000 {
        0o040000 => 'd',  // Directory
        0o120000 => 'l',  // Symbolic link
        0o100000 => '-',  // Regular file
        0o060000 => 'b',  // Block device
        0o020000 => 'c',  // Character device
        0o010000 => 'p',  // FIFO
        0o140000 => 's',  // Socket
        _ => '?',
    });
    
    // Owner permissions
    result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    
    // Group permissions  
    result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    
    // Other permissions
    result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    
    result
}

fn format_time(timestamp: i64) -> Result<String, Box<dyn std::error::Error>> {
    let system_time = UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64);
    let datetime = system_time.duration_since(UNIX_EPOCH)?;
    
    // Simple time formatting - just show seconds since epoch for now
    // In a full implementation, you'd format this as "YYYY-MM-DD HH:MM"
    let secs = datetime.as_secs();
    
    // Convert to a basic readable format
    // This is a simplified version - a real implementation would use proper date formatting
    let _days = secs / 86400;  // seconds per day
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    
    Ok(format!("{:02}:{:02}", hours, minutes))
}

fn is_executable(metadata: &fs::Metadata) -> bool {
    // Check if any execute bit is set
    metadata.mode() & 0o111 != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_mode() {
        // Test directory permissions
        let dir_mode = 0o040755; // drwxr-xr-x
        assert_eq!(format_mode(dir_mode), "drwxr-xr-x");
        
        // Test regular file permissions
        let file_mode = 0o100644; // -rw-r--r--
        assert_eq!(format_mode(file_mode), "-rw-r--r--");
        
        // Test executable file
        let exec_mode = 0o100755; // -rwxr-xr-x
        assert_eq!(format_mode(exec_mode), "-rwxr-xr-x");
    }
}
