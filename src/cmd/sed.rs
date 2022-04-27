use super::{Cmd, Args, Conf};

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "sed";
}

// 写一个比 sed 更好用的东西，sed 理解成本太高了
// 参考 https://github.com/chmln/sd，代码量并不大

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