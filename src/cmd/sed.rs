use super::{Cmd, CmdResult, Args, Conf};

pub(super) struct Command;

// 写一个比 sed 更好用的东西，sed 理解成本太高了
// 参考 https://github.com/chmln/sd，代码量并不大

const NAME: &'static str = "sed";

impl Cmd for self::Command {
    fn key(&self) -> String {
        NAME.to_string()
    }

    fn conf(&self) -> Conf {
        Conf::new(NAME)
    }

    fn process(&self, _args: &Args) -> CmdResult {
        println!("hello, my name is {}", NAME);
        Ok(())
    }
}