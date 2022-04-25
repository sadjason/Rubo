mod cmd;

use clap::{Command};
use crate::cmd::Container;

fn main() {
    let mut app = Command::new("rubo")
        .subcommand_required(true)
        .version("1.0")
        .author("Zhang Wei")
        .about("Rust Utilities");

    let mut container = Container::new();
    app.subcommands(container.commands());

    println!("hello, rubo, v5");
}
