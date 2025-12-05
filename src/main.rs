use std::{
    io::{self, Write},
    process,
};

use codecrafters_shell::{commands::Command, utils};

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
            } else if utils::find_os_executable(command).is_some() {
                let mut os_command = process::Command::new(command);

                // os_command.arg(command);
                for arg in args {
                    os_command.arg(arg);
                }

                let output = os_command.output()?;
                if output.status.success() {
                    io::stdout().write_all(&output.stdout)?;
                } else {
                    io::stderr().write_all(&output.stderr)?;
                }
            } else {
                println!("{}: command not found", command);
            }
        }
        Ok(())
    }
}
