mod base;

mod strip;
mod todo;
mod day;
mod sed;
mod count;
mod git;
mod lark;
mod ls;
mod cat;

use clap::{Arg, Command};

pub fn all_commands() -> Vec<Command<'static>> {
    vec![]
}
