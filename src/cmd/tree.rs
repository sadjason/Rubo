// https://github.com/solidiquis/erdtree
// - colorize: 颜色和 terminal 主题对应（即 ls 看到的颜色）
// - 对 tree 进行简化：
//   - L 默认 3
//   - du 开启

/// 开发要点：
/// - colorize，即 print 的 entry 和 `ls` 的颜色保持一致，
///   参考 https://gist.github.com/thomd/7667642，定义在 $LSCOLORS 中
/// - 简化 tree 参数：
///   - `-L` 默认值配置为 3
///   - `-du` 默认开启
///

use clap::arg;
use super::{Cmd, Args, Conf};
use std::env;
use std::fs;
use crate::lib::tree;

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "tree";
}

impl Cmd for Command {
    fn key(&self) -> String {
        Command::NAME.to_string()
    }

    fn conf(&self) -> Conf {
        Conf::new(Command::NAME)
            .arg_required_else_help(false)
            .about("List contents of directories in a tree-like format.")
    }

    fn process(&self, args: &Args) {
        let cur_dir = env::current_dir().unwrap();
        tree::walk(cur_dir);
    }
}
