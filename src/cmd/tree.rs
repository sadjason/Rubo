/// 开发要点：
/// - colorize，即 print 的 entry 和 `ls` 的颜色保持一致，
///   参考 https://gist.github.com/thomd/7667642，定义在 $LSCOLORS 中
/// - 简化 tree 参数：
///   - `-L` 默认值配置为 3
///   - `-du` 默认开启
/// 借鉴了点 https://github.com/solidiquis/erdtree，但感觉比这个做得好一些

use std::env;
use std::path::{PathBuf};
use clap::arg;
use crate::cmd::{Cmd, CmdResult, Args, Conf};
use crate::lib::{tree, util::walker::Walker};

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "tree";
}

impl Cmd for Command {
    fn key(&self) -> String {
        Command::NAME.to_string()
    }

    fn conf(&self) -> Conf {
        // -l     Follows symbolic links if they point to directories, as if they were directories.
        // Symbolic links that will result in recursion are avoided when detected.
        Conf::new(Command::NAME)
            .arg_required_else_help(false)
            .arg(
                arg!(-p --path <PATH> "Path to directory. Defaults to current working directory")
                    .required(false)
            )
            .arg(arg!(-L --level <LEVEL> "Max display depth of the directory tree")
                .required(false)
            )
            .arg(arg!(-a --all "All files are printed. By default tree does not print hidden files (those beginning with a dot '.')")
                .required(false)
            )
            .arg(arg!(-l --"follow-symbolic" "Follows symbolic links if they point to directories, as if they were directories. Symbolic links that will result in recursion are avoided when detected")
                .required(false)
            )
            .about("List contents of directories in a tree-like format.")
    }

    fn process(&self, args: &Args) -> CmdResult {
        // parse `path`
        let dir =
            if let Some(p) = args.value_of("path") {
                PathBuf::from(p).to_path_buf()
            } else {
                env::current_dir().unwrap_or(PathBuf::from("."))
            };
        let mut walker = Walker::new(dir);

        // parse `depth`
        let depth =
            if let Some(d) = args.value_of("level") {
                d.parse::<usize>().ok()
            } else {
                None
            };
        walker.max_depth(depth);

        // parse `all`
        walker.ignore_hidden(args.occurrences_of("all") < 1);

        // parse `follow-symbolic`
        walker.follow_symbolic(args.occurrences_of("follow-symbolic") > 0);

        tree::walk(walker)
    }
}
