use std::collections::HashMap;
use std::boxed::Box;
use clap::{Command, ArgMatches};

type Conf = Command<'static>;
type Args = ArgMatches;

trait Cmd {
    fn key(&self) -> String;
    // 关于 command 的配置
    fn conf(&self) -> Conf;
    // 处理
    fn process(&self, args: &Args);
}

// commands
mod ls;
mod cat;
mod day;
mod pod;
mod sed;
mod git;
mod lark;
mod todo;
mod count;
mod strip;

pub struct Container {
    commands: HashMap<String, Box<dyn Cmd>>
}

impl Container {
    pub fn new() -> Self {
        Container { commands: HashMap::new() }
    }

    // TODO: 优雅的方式注册 command
    pub fn commands(&mut self) -> Vec<Command<'static>> {
        let cmd = Box::new(sed::Command);
        let c1 = cmd.conf();
        self.commands.insert(cmd.key(), cmd);
        vec![c1]
    }

    // TODO: error 处理
    pub fn process(&self, args: Args) {
        match args.subcommand() {
            Some((sub_cmd, sub_args)) => {
                let key = sub_cmd.to_string();
                let cmd = self.commands.get(&key).unwrap();
                cmd.process(sub_args);
            },
            None => {

            }
        }
    }
}
