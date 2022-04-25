use std::collections::HashMap;
use std::boxed::Box;
use clap::{Command, ArgMatches};

type Conf = Command<'static>;
type Args = ArgMatches;

trait Cmd {
    fn key(&self) -> String;
    fn conf(&self) -> Conf;
    fn process(&self, args: &Args) -> bool;
}

mod strip;
mod todo;
mod day;
mod sed;
mod count;
mod git;
mod lark;
mod ls;
mod cat;

pub struct Container {
    commands: HashMap<String, Box<dyn Cmd>>
}

pub struct Foo;

impl Container {
    pub fn new() -> Self {
        Container { commands: HashMap::new() }
    }

    pub fn commands(&mut self) -> Vec<Command<'static>> {
        let mut vec = Vec::new();
        let cmd = Box::new(sed::command());
        {
            let c1 = cmd.as_ref().conf();
            vec.push(c1);
        }
        self.commands.insert(cmd.key(), cmd);
        vec
    }

    pub fn process(&self, _args: Args) {

    }
}
