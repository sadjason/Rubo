use super::{Cmd, CmdResult, Args, Conf};

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "cd";
}

impl Cmd for Command {
    fn key(&self) -> String {
        Command::NAME.to_string()
    }

    fn conf(&self) -> Conf {
        Conf::new(Command::NAME)
            .arg_required_else_help(true)
            .about("just like wd")
    }

    fn process(&self, _args: &Args) -> CmdResult {
        // 类似于 `wd`
        // 参考 cd
        Ok(())
    }
}