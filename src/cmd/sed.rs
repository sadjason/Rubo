use super::{Cmd, Args, Conf};

pub fn command() -> impl Cmd {
    Command {}
}

struct Command;

// 写一个比 sed 更好用的东西，sed 理解成本太高了
// 参考 https://github.com/chmln/sd，代码量并不大

impl super::Cmd for Command {
    fn key(&self) -> String { "sed".to_string() }
    fn conf(&self) -> Conf {
        panic!("panic");
    }
    fn process(&self, _args: &Args) -> bool {
        false
    }
}