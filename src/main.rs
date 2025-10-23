#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // REPL
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        // Print error
        println!("{}: command not found", command.trim());
    }
}
