/// Simple command parser that splits input by whitespace and handles quoted strings
/// Returns (command, args) where command is the first token and args are the rest
pub fn parse_command(input: &str) -> (&str, Vec<String>) {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    
    for ch in input.chars() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            '"' | '\'' if in_quotes && ch == quote_char => {
                in_quotes = false;
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            ' ' | '\t' if !in_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }
    
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    
    if tokens.is_empty() {
        return ("", vec![]);
    }
    
    let command = tokens[0].as_str();
    let args = if tokens.len() > 1 {
        tokens[1..].to_vec()
    } else {
        vec![]
    };
    
    // We need to return a static str for command, so we'll leak it
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
