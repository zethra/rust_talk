use std::env;

use unescape::unescape;

fn main() {
    // Create a string buffer
    let mut buffer = String::new();
    // Get the cli args iterator skipping the first arg
    let args = env::args().skip(1);
    // Get the length of the
    let len = args.len();
    // Iterate over the cli args
    for (i, arg) in args.enumerate() {
        // Push the arg into the buffer
        buffer.push_str(&arg);
        // If it's not the last arg also push a space
        if i < len - 1 {
            buffer.push(' ');
        }
    }
    // Print the escaped result
    match unescape(&buffer) {
        Some(s) => println!("{}", s),
        None => eprintln!("Invalid escape sequence")
    }
}
