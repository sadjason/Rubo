use super::{Cmd, Args, Conf};

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "pod";
}

impl Cmd for Command {
    fn key(&self) -> String {
        Command::NAME.to_string()
    }

    fn conf(&self) -> Conf {
        Conf::new(Command::NAME)
    }

    fn process(&self, _args: &Args) {
        println!("hello, my name is {}", Command::NAME);
    }
}