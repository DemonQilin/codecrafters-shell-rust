use std::io::{self, Write};

use codecrafters_shell::commands::{EchoCommand, Executable, ExitCommand};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed = line.trim();
        let (command, args): (&str, Vec<&str>) = {
            if let Some((command, rest_line)) = trimmed.split_once(" ") {
                (command, rest_line.split_ascii_whitespace().collect())
            } else {
                (trimmed, Vec::new())
            }
        };

        match command {
            EchoCommand::NAME => EchoCommand::execute(&args),
            ExitCommand::NAME => ExitCommand::execute(&args),
            _ => println!("{}: command not found", command),
        };
    }
}
