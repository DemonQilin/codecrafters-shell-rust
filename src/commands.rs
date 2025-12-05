use std::{fmt, process};

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
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = CommandError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "echo" => Ok(Self::Echo),
            "exit" => Ok(Self::Exit),
            _ => Err(CommandError::NotFound(value.to_owned())),
        }
    }
}

impl Command {
    pub fn run(&self, args: &[&str]) {
        match self {
            Command::Echo => run_echo(args),
            Command::Exit => run_exit(args.first().copied()),
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
