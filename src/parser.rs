/// Simple command parser that splits input by whitespace
/// Returns (command, args) where command is the first token and args are the rest
pub fn parse_command(input: &str) -> (&str, Vec<&str>) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    if tokens.is_empty() {
        return ("", vec![]);
    }
    
    let command = tokens[0];
    let args = if tokens.len() > 1 {
        tokens[1..].to_vec()
    } else {
        vec![]
    };
    
    (command, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_input() {
        let (cmd, args) = parse_command("");
        assert_eq!(cmd, "");
        assert_eq!(args, Vec::<&str>::new());
    }
    
    #[test]
    fn test_single_command() {
        let (cmd, args) = parse_command("pwd");
        assert_eq!(cmd, "pwd");
        assert_eq!(args, Vec::<&str>::new());
    }
    
    #[test]
    fn test_command_with_args() {
        let (cmd, args) = parse_command("echo hello world");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello", "world"]);
    }
}
