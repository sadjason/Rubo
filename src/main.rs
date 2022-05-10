mod cmd;
mod lib;

use clap::Command;
use crate::cmd::Container;
use crate::lib::pod::lockfile::Lockfile;

fn main() {
    let mut container = Container::new();
    let app = Command::new("rubo")
        .subcommand_required(true)
        .subcommands(container.commands())
        .version("1.0")
        .author("Zhang Wei")
        .about("Rust Utilities");
    container.process(app.get_matches());
}
