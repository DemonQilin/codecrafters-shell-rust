use std::io::{self, Write};

use codecrafters_shell::commands::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shell = Shell;

    shell.run()
}

struct Shell;

impl Shell {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            print!("$ ");
            io::stdout().flush()?;

            let mut line = String::new();
            let bytes_read = io::stdin().read_line(&mut line)?;

            // Handle EOF (Ctrl+D)
            if bytes_read == 0 {
                println!();
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let (command, args): (&str, Vec<&str>) = {
                if let Some((command, rest_line)) = trimmed.split_once(" ") {
                    (command, rest_line.split_ascii_whitespace().collect())
                } else {
                    (trimmed, Vec::new())
                }
            };

            if let Ok(command) = Command::try_from(command) {
                command.run(&args);
            } else {
                println!("{}: command not found", command)
            }
        }
        Ok(())
    }
}
