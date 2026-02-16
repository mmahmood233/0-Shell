//! # echo - Display Text
//!
//! Prints arguments to standard output, separated by spaces.

/// Executes the `echo` command.
///
/// Prints all arguments to stdout, separated by single spaces, followed by a newline.
/// If no arguments are provided, prints an empty line.
///
/// # Arguments
/// * `args` - Vector of strings to print
///
/// # Examples
/// ```
/// $ echo hello world
/// hello world
///
/// $ echo "Hello World"
/// Hello World
/// ```
///
/// # Note
/// Quote handling is performed by the parser before this function is called.
pub fn execute(args: &[String]) {
    // Handle empty arguments - just print newline
    if args.is_empty() {
        println!();
        return;
    }
    
    // Join all arguments with single spaces and print
    let output = args.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
    println!("{}", output);
}
