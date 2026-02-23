//! # ls - List Directory Contents
//!
//! Lists files and directories with support for various display options.

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

/// Flags for controlling ls output format.
#[derive(Default)]
struct LsFlags {
    show_all: bool,      // -a flag: show hidden files (starting with .)
    long_format: bool,   // -l flag: long format with details
    classify: bool,      // -F flag: append indicators (/, *, etc.)
}

/// Executes the `ls` command.
///
/// Lists the contents of a directory with optional formatting flags.
/// Flags can be combined (e.g., `-la`, `-lF`, `-laF`).
///
/// # Arguments
/// * `args` - Command arguments (flags and optional directory path)
///
/// # Supported Flags
/// - `-a` - Show all files, including hidden files (starting with `.`)
/// - `-l` - Long format: permissions, links, owner, size, time, name
/// - `-F` - Classify: append `/` to directories, `*` to executables
///
/// # Examples
/// ```
/// $ ls
/// file1.txt
/// dir1
///
/// $ ls -l
/// -rw-r--r--   1 501:20      100 16:00 file1.txt
/// drwxr-xr-x   2 501:20       64 16:00 dir1/
///
/// $ ls -a
/// .
/// ..
/// .hidden
/// file1.txt
///
/// $ ls -F
/// file1.txt
/// dir1/
/// script.sh*
/// ```
///
/// # Errors
/// Prints error messages to stderr for:
/// - Invalid flag
/// - Directory does not exist
/// - Permission denied
pub fn execute(args: &[String]) {
    // Initialize flags and default path
    let mut flags = LsFlags::default();
    let mut path = ".";  // Default to current directory
    
    // Parse command arguments (flags and path)
    for arg in args.iter() {
        if arg.starts_with('-') {
            // Argument is a flag - parse each character after the dash
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
            // Argument is a path - use it as the directory to list
            path = arg.as_str();
        }
    }
    
    // List the directory contents with the specified flags
    if let Err(e) = list_directory(path, &flags) {
        eprintln!("ls: {}: {}", path, e);
    }
}

fn list_directory(path: &str, flags: &LsFlags) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = Path::new(path);
    
    // Canonicalize the path to get absolute path for proper parent resolution
    let canonical_path = fs::canonicalize(dir_path)?;
    let entries = fs::read_dir(dir_path)?;
    
    let mut items = Vec::new();
    let mut total_blocks = 0u64;
    
    // Add . and .. entries when -a flag is set
    if flags.show_all {
        // Add current directory (.)
        if let Ok(metadata) = fs::metadata(&canonical_path) {
            total_blocks += metadata.blocks();
            items.push((".".to_string(), canonical_path.clone(), metadata));
        }
        
        // Add parent directory (..)
        // Get parent of canonical path
        let parent_path = if let Some(parent) = canonical_path.parent() {
            parent.to_path_buf()
        } else {
            // At root, parent is self
            canonical_path.clone()
        };
        
        if let Ok(metadata) = fs::metadata(&parent_path) {
            total_blocks += metadata.blocks();
            items.push(("..".to_string(), parent_path, metadata));
        }
    }
    
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
        // Count actual blocks allocated (512-byte blocks on Unix)
        total_blocks += metadata.blocks();
        items.push((name.to_string(), entry.path(), metadata));
    }
    
    // Sort by name
    items.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Print total line in long format
    if flags.long_format && !items.is_empty() {
        println!("total {}", total_blocks);
    }
    
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

fn print_long_format(name: &str, path: &Path, metadata: &fs::Metadata) -> Result<(), Box<dyn std::error::Error>> {
    // Format: mode links user group size mtime name
    let mode_str = format_mode(metadata.mode());
    let nlink = metadata.nlink();
    let uid = metadata.uid();
    let gid = metadata.gid();
    
    // Get username and group name (fallback to UID:GID if not available)
    let user = get_username(uid).unwrap_or_else(|| uid.to_string());
    let group = get_groupname(gid).unwrap_or_else(|| gid.to_string());
    
    let size = metadata.size();
    let mtime = format_time(metadata.mtime())?;
    
    // Check for extended attributes (macOS)
    let has_xattr = has_extended_attributes(path);
    let xattr_marker = if has_xattr { "@" } else { "" };
    
    let mut display_name = name.to_string();
    
    // Add classifier for directories and executables
    if metadata.is_dir() {
        display_name.push('/');
    } else if is_executable(metadata) {
        display_name.push('*');
    }
    
    // Format similar to system ls: permissions[@] links user group size date name
    // Right-align numeric fields, left-align text fields
    println!("{}{} {:>3} {:<8} {:<8} {:>8} {} {}", 
             mode_str, xattr_marker, nlink, user, group, size, mtime, display_name);
    
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
    // Convert Unix timestamp to a readable date format
    // Format: "Mon DD HH:MM" (e.g., "Feb 15 19:09")
    
    const SECONDS_PER_DAY: i64 = 86400;
    const SECONDS_PER_HOUR: i64 = 3600;
    const SECONDS_PER_MINUTE: i64 = 60;
    
    // Get local timezone offset
    let local_timestamp = timestamp + get_timezone_offset();
    
    // Days since Unix epoch (Jan 1, 1970) in local time
    let days_since_epoch = local_timestamp / SECONDS_PER_DAY;
    
    // Calculate year, month, day (simplified algorithm)
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;
    
    // Advance years
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    // Determine month and day
    let days_in_months = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    
    let mut month = 0;
    let mut day = remaining_days + 1;
    
    for (i, &days) in days_in_months.iter().enumerate() {
        if day <= days {
            month = i;
            break;
        }
        day -= days;
    }
    
    // Calculate time in local timezone
    let time_of_day = local_timestamp % SECONDS_PER_DAY;
    let hours = ((time_of_day / SECONDS_PER_HOUR) % 24) as u32;
    let minutes = ((time_of_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u32;
    
    // Month names
    let month_names = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                       "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    
    Ok(format!("{} {:2} {:02}:{:02}", month_names[month], day, hours, minutes))
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_timezone_offset() -> i64 {
    // Get the local timezone offset in seconds
    // Use localtime_r to get tm_gmtoff which contains the offset
    unsafe {
        // Get current time
        let now = libc::time(std::ptr::null_mut());
        
        // Convert to local time structure
        let mut tm: libc::tm = std::mem::zeroed();
        let result = libc::localtime_r(&now, &mut tm);
        
        if result.is_null() {
            return 0;
        }
        
        // tm_gmtoff contains the offset from UTC in seconds
        // This field is available on macOS/BSD systems
        tm.tm_gmtoff as i64
    }
}

fn get_username(uid: u32) -> Option<String> {
    use std::env;
    
    // Try to get current user's UID and username from environment
    // This works for files owned by the current user
    if let Ok(user) = env::var("USER") {
        // Check if this file is owned by current user
        // We can get current UID from the environment or by checking a known file
        if let Ok(current_dir) = env::current_dir() {
            if let Ok(metadata) = std::fs::metadata(&current_dir) {
                use std::os::unix::fs::MetadataExt;
                if metadata.uid() == uid {
                    return Some(user);
                }
            }
        }
    }
    
    None
}

fn get_groupname(gid: u32) -> Option<String> {
    use std::env;
    
    // Try to determine if this is the user's primary group
    // On macOS, staff (gid 20) is the default group for regular users
    if gid == 20 {
        return Some("staff".to_string());
    }
    
    // Check if it matches current directory's group
    if let Ok(current_dir) = env::current_dir() {
        if let Ok(metadata) = std::fs::metadata(&current_dir) {
            use std::os::unix::fs::MetadataExt;
            if metadata.gid() == gid {
                // Common group names for typical GIDs
                return match gid {
                    0 => Some("wheel".to_string()),
                    20 => Some("staff".to_string()),
                    _ => None,
                };
            }
        }
    }
    
    None
}

fn is_executable(metadata: &fs::Metadata) -> bool {
    // Check if any execute bit is set
    metadata.mode() & 0o111 != 0
}

fn has_extended_attributes(path: &Path) -> bool {
    // Check if file has extended attributes (macOS xattr)
    // We use a simple heuristic: try to list xattrs using std library
    // On macOS, most files have com.apple.* extended attributes
    
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    
    // Convert path to C string
    if let Ok(c_path) = CString::new(path.as_os_str().as_bytes()) {
        // Use libc to check for extended attributes
        // listxattr returns the size of the attribute list
        #[cfg(target_os = "macos")]
        unsafe {
            let size = libc::listxattr(
                c_path.as_ptr(),
                std::ptr::null_mut(),
                0,
                0
            );
            return size > 0;
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // On non-macOS systems, we don't show the @ symbol
            let _ = c_path; // Suppress unused warning
            return false;
        }
    }
    
    false
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
