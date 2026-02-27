use std::{
    io::{self, Write},
    process,
};

use codecrafters_shell::{commands::Command, utils};

fn main() -> anyhow::Result<()> {
    run_shell()
}

fn run_shell() -> anyhow::Result<()> {
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
            let mut parts = trimmed.split_ascii_whitespace();
            let command = match parts.next() {
                Some(cmd) => cmd,
                None => continue,
            };
            let args: Vec<&str> = parts.collect();

            (command, args)
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
