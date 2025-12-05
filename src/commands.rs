use std::{env, fmt, os::unix::fs::PermissionsExt, process};

#[derive(Debug)]
pub enum CommandError {
    NotFound(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::NotFound(value) => write!(f, "{value}: command not found"),
        }
    }
}

impl std::error::Error for CommandError {}

#[derive(Debug)]
pub enum Command {
    Echo,
    Exit,
    Type,
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = CommandError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "echo" => Ok(Self::Echo),
            "exit" => Ok(Self::Exit),
            "type" => Ok(Self::Type),
            _ => Err(CommandError::NotFound(value.to_owned())),
        }
    }
}

impl Command {
    pub fn run(&self, args: &[&str]) {
        match self {
            Command::Echo => run_echo(args),
            Command::Exit => run_exit(args.first().copied()),
            Command::Type => run_type(args.first().copied()),
        };
    }
}

fn run_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn run_exit(arg: Option<&str>) {
    let exit_code = if let Some(value) = arg {
        value.parse::<i32>().unwrap_or(1)
    } else {
        0
    };

    process::exit(exit_code);
}

fn run_type(arg: Option<&str>) {
    if arg.is_none() {
        eprintln!("Required argument: type <command>");
        return;
    }

    let arg = arg.unwrap();
    let command: Result<Command, _> = arg.try_into();
    if command.is_ok() {
        println!("{} is a shell builtin", arg);
        return;
    }

    let found = env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(arg);

            if full_path.is_file() {
                let is_executable = full_path
                    .metadata()
                    .is_ok_and(|m| m.permissions().mode() & 0o111 != 0);

                if is_executable {
                    return Some(full_path);
                }
            }

            None
        })
    });

    if let Some(path) = found {
        println!("{arg} is {}", path.display());
    } else {
        println!("{}: not found", arg)
    }
}
