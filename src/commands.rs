use std::process;

pub trait Executable {
    const NAME: &'static str;

    fn execute(args: &[&str]);
}

pub struct EchoCommand;
impl Executable for EchoCommand {
    const NAME: &'static str = "echo";

    fn execute(args: &[&str]) {
        println!("{}", args.join(" "));
    }
}

pub struct ExitCommand;
impl Executable for ExitCommand {
    const NAME: &'static str = "exit";

    fn execute(args: &[&str]) {
        let exit_code = if args.len() > 1 {
            args[1].parse::<i32>().unwrap_or(1)
        } else {
            0
        };
        process::exit(exit_code);
    }
}
