// 用于统计

// 统计行数，ref: https://github.com/XAMPPRocky/tokei
// ru count line
// 统计大小，ref: https://github.com/bootandy/dust/
// ru count size
// 统计时间，ref: https://github.com/sharkdp/hyperfine
// ru count time

use super::{Cmd, Args, Conf};

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "count";
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