//! # Command Parser Module
//!
//! Provides quote-aware parsing of shell command input strings.
//! Handles both single (`'`) and double (`"`) quotes, preserving spaces
//! within quoted strings while properly tokenizing unquoted input.

/// Parses a command line input string into a command name and arguments.
///
/// This parser handles:
/// - Whitespace-separated tokens (space and tab)
/// - Single-quoted strings (`'...'`)
/// - Double-quoted strings (`"..."`)
/// - Preserves spaces within quoted strings
/// - Strips quotes from the output
///
/// # Arguments
/// * `input` - The raw command line string to parse
///
/// # Returns
/// A tuple containing:
/// - `&str` - The command name (first token)
/// - `Vec<String>` - Vector of argument strings (remaining tokens)
///
/// # Examples
/// ```
/// let (cmd, args) = parse_command("echo hello world");
/// assert_eq!(cmd, "echo");
/// assert_eq!(args, vec!["hello", "world"]);
///
/// let (cmd, args) = parse_command("echo \"Hello World\"");
/// assert_eq!(cmd, "echo");
/// assert_eq!(args, vec!["Hello World"]);
/// ```
///
/// # Quote Handling
/// - Opening quote (`"` or `'`) starts a quoted string
/// - Closing quote (matching the opening quote) ends the quoted string
/// - Spaces and tabs inside quotes are preserved
/// - Quotes are removed from the final tokens
/// - Mismatched quotes are treated as regular characters
///
/// # Memory Note
/// The command string is leaked (using `Box::leak`) to provide a `'static` lifetime.
/// This is acceptable for a long-running shell process where commands are continuously
/// parsed throughout the program's lifetime.
pub fn parse_command(input: &str) -> (&str, Vec<String>) {
    // Vector to store parsed tokens
    let mut tokens = Vec::new();
    // Current token being built
    let mut current_token = String::new();
    // Track if we're inside a quoted string
    let mut in_quotes = false;
    // Remember which quote character started the quoted string
    let mut quote_char = ' ';
    
    // Iterate through each character in the input
    for ch in input.chars() {
        match ch {
            // Opening quote (double or single) when not already in quotes
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            // Closing quote (matching the opening quote)
            '"' | '\'' if in_quotes && ch == quote_char => {
                in_quotes = false;
                // Save the quoted token if it's not empty
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            // Whitespace (space or tab) when not in quotes - token separator
            ' ' | '\t' if !in_quotes => {
                // Save the current token if it's not empty
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            // Any other character - add to current token
            _ => {
                current_token.push(ch);
            }
        }
    }
    
    // Save any remaining token
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    
    // Handle empty input
    if tokens.is_empty() {
        return ("", vec![]);
    }
    
    // First token is the command name
    let command = tokens[0].as_str();
    // Remaining tokens are arguments
    let args = if tokens.len() > 1 {
        tokens[1..].to_vec()
    } else {
        vec![]
    };
    
    // Convert command to 'static lifetime by leaking memory
    // This is acceptable for a shell that runs continuously
    let command_static = Box::leak(command.to_string().into_boxed_str());
    
    (command_static, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_input() {
        let (cmd, args) = parse_command("");
        assert_eq!(cmd, "");
        assert_eq!(args, Vec::<String>::new());
    }
    
    #[test]
    fn test_single_command() {
        let (cmd, args) = parse_command("pwd");
        assert_eq!(cmd, "pwd");
        assert_eq!(args, Vec::<String>::new());
    }
    
    #[test]
    fn test_command_with_args() {
        let (cmd, args) = parse_command("echo hello world");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello".to_string(), "world".to_string()]);
    }
    
    #[test]
    fn test_quoted_strings() {
        let (cmd, args) = parse_command("echo \"Hello World\"");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["Hello World".to_string()]);
    }
    
    #[test]
    fn test_single_quotes() {
        let (cmd, args) = parse_command("echo 'Hello World'");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["Hello World".to_string()]);
    }
}
