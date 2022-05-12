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
mod cd;
mod ls;
mod cat;
mod day;
mod git;
mod pod;
mod sed;
mod help;
mod lark;
mod life;
mod open;
mod todo;
mod tree;
mod work;
mod count;
mod strip;

pub struct Container {
    commands: HashMap<String, Box<dyn Cmd>>
}

impl Container {
    pub fn new() -> Self {
        Container { commands: HashMap::new() }
    }

    pub fn commands(&mut self) -> Vec<Conf> {
        let mut vec = Vec::new();
        self.add_cmd(pod::Command, &mut vec);
        self.add_cmd(tree::Command, &mut vec);
        vec
    }

    fn add_cmd(&mut self, cmd: impl Cmd + 'static, vec: &mut Vec<Command<'static>>) {
        let cmd = Box::new(cmd);
        let conf = cmd.conf();
        self.commands.insert(cmd.key(), cmd);
        vec.push(conf);
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
