use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed = line.trim();

        if trimmed.starts_with("exit") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let exit_code = if parts.len() > 1 {
                parts[1].parse::<i32>().unwrap_or(1)
            } else {
                0
            };
            process::exit(exit_code);
        } else {
            println!("{}: command not found", trimmed);
        }
    }
}
