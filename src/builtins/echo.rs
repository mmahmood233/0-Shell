/// Echo command - prints arguments separated by spaces
/// Basic implementation without advanced quoting support
pub fn execute(args: &[String]) {
    if args.is_empty() {
        println!();
        return;
    }
    
    // Join arguments with single spaces and print
    let output = args.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
    println!("{}", output);
}
