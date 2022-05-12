use std::path::PathBuf;
use clap::arg;
use crate::cmd::{Cmd, Args, Conf};
use crate::lib::pod::dep;

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "pod";

    const SUB_DEP: &'static str = "dep";
    const SUB_RDEP: &'static str = "rdep";
    const SUB_SEARCH: &'static str = "search";
    const SUB_CLEAN: &'static str = "clean";

    fn sub_cmd_conf_list(&self) -> Vec<Conf> {
        let arg_name = || { arg!(-n --name <NAME> "Pod name").required(true) };
        let arg_path = || { arg!(-p --path <PATH> "Path to Podfile.lock").required(false) };
        let arg_depth = || { arg!(-d --depth <DEPTH> "Max display depth").required(false) };
        let dep = Conf::new(Command::SUB_DEP)
            .args(&[arg_name(), arg_path(), arg_depth()])
            .about("Find dependencies for specified pod");

        let rdep = Conf::new(Command::SUB_RDEP)
            .args(&[arg_name(), arg_path(), arg_depth()])
            .about("Find reserve dependencies for specified pod");

        // let arg_text = || { arg!(-t --text <TEXT> "Search pattern").required(true) };
        // let arg_exclude = || { arg!(-e --excludes <POD_NAMES> "exclude pod names").required(false) };
        // let arg_a = || { arg!(-A --"after-context" <NUM> "show NUM lines after each match").required(false) };
        // let arg_b = || { arg!(-B --"before-context" <NUM> "Show NUM lines before each match").required(false) };
        // let arg_name_only = || { arg!(--"name-only" "only display pod name").required(false) };
        // let search = Conf::new(Command::SUB_SEARCH)
        //     .args(&[arg_text(), arg_exclude(), arg_a(), arg_b(), arg_name_only()])
        //     .about("Search in Pods/ directory");
        //
        // let clean = Conf::new(Command::SUB_CLEAN)
        //     .about("Clean pods and free disk");
        // vec![dep, rdep, search, clean]
        vec![dep, rdep]
    }
}

impl Cmd for Command {
    fn key(&self) -> String {
        Command::NAME.to_string()
    }

    fn conf(&self) -> Conf {
        Conf::new(Command::NAME)
            .arg_required_else_help(true)
            .subcommands(self.sub_cmd_conf_list())
            .about("Pod Utilities")
    }

    fn process(&self, args: &Args) {
        let (sub_cmd, sub_args) = args.subcommand().unwrap();
        match sub_cmd {
            Command::SUB_DEP | Command::SUB_RDEP => {
                let path =
                    if let Some(p) = sub_args.value_of("path") {
                        PathBuf::from(p).to_path_buf()
                    } else {
                        let mut p = std::env::current_dir()
                            .unwrap_or(PathBuf::from("."));
                        p.push("Podfile.lock");
                        p
                    };
                let target= sub_args.value_of("name").unwrap();
                let max_depth = sub_args.value_of("depth")
                    .and_then(|d| d.to_string().parse::<usize>().ok())
                    .unwrap_or(999_999_999);
                if let Command::SUB_DEP = sub_cmd {
                    dep::print_deps(path, target, max_depth).unwrap();
                } else {
                    dep::print_reserve_deps(path, target, max_depth).unwrap();
                }
            },
            Command::SUB_SEARCH => {
                // rob.arg(Command::SUB_SEARCH);
                // if let Some(text) = sub_args.value_of("text") {
                //     rob.arg("--text").arg(text);
                // }
                // if let Some(excludes) = sub_args.value_of("excludes") {
                //     rob.arg("--excludes").arg(excludes);
                // }
                // if let Some(before) = sub_args.value_of("before-context") {
                //     rob.arg("-B").arg(before);
                // }
                // if let Some(after) = sub_args.value_of("after-context") {
                //     rob.arg("-A").arg(after);
                // }
                // if sub_args.occurrences_of("name-only") > 0 {
                //     rob.arg("--name-only");
                // }
                // // 中转命令到 `rob pod`
                // let mut rob = process::Command::new("rob");
                // rob.arg("pod");
                // rob.spawn().unwrap().wait().unwrap();
                println!("暂时未实现");
            },
            Command::SUB_CLEAN => {
                // rob.arg(Command::SUB_CLEAN);
                // 策略一：基于 last-access 处理
                // 策略二：基于版本号（譬如低于 5.11 版本）
                // 策略三：基于当前项目中使用的使用状况
                println!("暂时未实现");
            },
            _ => {

            }
        }
    }
}