use std::{env, path::Path, process};

use crate::utils;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{0}: command not found")]
    NotFound(String),
}

#[derive(Debug)]
pub enum Command {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = CommandError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "echo" => Ok(Self::Echo),
            "exit" => Ok(Self::Exit),
            "type" => Ok(Self::Type),
            "pwd" => Ok(Self::Pwd),
            "cd" => Ok(Self::Cd),
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
            Command::Pwd => run_pwd(),
            Command::Cd => run_cd(args.first().copied()),
        };
    }
}

fn run_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn run_exit(arg: Option<&str>) {
    let exit_code = arg.map_or(0, |v| v.parse::<i32>().unwrap_or(1));

    process::exit(exit_code);
}

fn run_type(arg: Option<&str>) {
    let Some(arg) = arg else {
        eprintln!("Required argument: type <command>");
        return;
    };

    if Command::try_from(arg).is_ok() {
        println!("{} is a shell builtin", arg);
        return;
    }

    if let Some(path) = utils::find_os_executable(arg) {
        println!("{arg} is {}", path.display());
    } else {
        println!("{}: not found", arg)
    }
}

fn run_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {e}"),
    }
}

fn run_cd(arg: Option<&str>) {
    let new_path = match arg {
        Some(a) if a != "~" => Path::new(a).to_path_buf(),
        _ => match env::home_dir() {
            Some(home_dir) => home_dir,
            None => {
                eprint!("cd: HOME dir not available");
                return;
            }
        },
    };

    if !new_path.exists() {
        eprintln!("cd: {}: No such file or directory", new_path.display());
        return;
    };

    if new_path.is_dir() {
        if let Err(e) = env::set_current_dir(new_path) {
            eprintln!("cd: {e}");
        };
    }
}
