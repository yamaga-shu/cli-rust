use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("cat").about("Rust cat")
}

pub fn run(_matches: &ArgMatches) {
    // TODO: Implement cat
}
