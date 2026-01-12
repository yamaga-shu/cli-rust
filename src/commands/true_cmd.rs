use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("true").about("Rust true")
}

pub fn run(_matches: &ArgMatches) {
    // Do nothing - exit successfully
}
