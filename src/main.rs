#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

use anyhow::Ok;

fn main() {
    // REPL
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        // Handle input
        if line.trim().starts_with("exit") {
            let arg = line
                .trim()
                .split(' ')
                .collect::<Vec<_>>()
                .get(1)
                .map_or(0, |arg| arg.parse().or(Ok(1)).unwrap());

            if arg != 0 {
                process::exit(1);
            } else {
                process::exit(0);
            }
        }

        // Print error
        println!("{}: command not found", line.trim());
    }
}
