mod command;

use clap::{Arg, Command};

fn main() {
    let mut app = Command::new("rubo")
        .subcommand_required(true)
        .version("1.0")
        .author("Zhang Wei")
        .about("Rust Utilities");

    app.subcommands(command::all_commands());

    println!("hello, rubo, v2");
}
