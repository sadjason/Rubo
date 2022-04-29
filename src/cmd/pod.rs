use clap::arg;
use super::{Cmd, Args, Conf};
use std::process;

pub(super) struct Command;

impl Command {
    const NAME: &'static str = "pod";

    fn sub_cmd_conf_list(&self) -> Vec<Conf> {
        let arg_name = || {
            arg!(-n --name <NAME> "Pod name").required(true)
        };
        let arg_path = || {
            arg!(-p --path <PATH> "Path to Podfile.lock").required(false)
        };
        let arg_depth = || {
            arg!(-d --depth <DEPTH> "Max display depth").required(false)
        };
        let dep = Conf::new("dep")
            .args(&[arg_name(), arg_path(), arg_depth()])
            .about("Find dependencies for specified pod");

        let rdep = Conf::new("rdep")
            .args(&[arg_name(), arg_path(), arg_depth()])
            .about("Find reserve dependencies for specified pod");

        let arg_text = || {
            arg!(-t --text <TEXT> "Search pattern").required(true)
        };
        let arg_exclude = || {
            arg!(-e --excludes <POD_NAMES> "exclude pod names").required(false)
        };
        let arg_name_only = || {
            arg!(--"name-only" "only display pod name").required(false)
        };
        let search = Conf::new("search")
            .args(&[arg_text(), arg_exclude(), arg_name_only()])
            .about("Search in Pods/ directory");
        vec![dep, rdep, search]
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
        let mut rob = process::Command::new("rob");
        rob.arg("pod");
        match sub_cmd {
            "dep" => {
                rob.arg("dep");
                if let Some(pod) = sub_args.value_of("name") {
                    rob.arg("--name").arg(pod);
                }
                if let Some(path) = sub_args.value_of("path") {
                    rob.arg("--path").arg(path);
                }
                if let Some(depth) = sub_args.value_of("depth") {
                    rob.arg("--depth").arg(depth);
                }
            },
            "rdep" => {
                rob.arg("rdep");
                if let Some(pod) = sub_args.value_of("name") {
                    rob.arg("--name").arg(pod);
                }
                if let Some(path) = sub_args.value_of("path") {
                    rob.arg("--path").arg(path);
                }
                if let Some(depth) = sub_args.value_of("depth") {
                    rob.arg("--depth").arg(depth);
                }
            },
            "search" => {
                rob.arg("search");
                if let Some(text) = sub_args.value_of("text") {
                    rob.arg("--text").arg(text);
                }
                if let Some(excludes) = sub_args.value_of("excludes") {
                    rob.arg("--excludes").arg(excludes);
                }
                if sub_args.occurrences_of("name-only") > 0 {
                    rob.arg("--name-only");
                }
            },
            _ => {

            }
        }
        rob.spawn().unwrap().wait();
    }
}