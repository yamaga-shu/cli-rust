use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("false").about("Rust false")
}

pub fn run(_matches: &ArgMatches) {
    std::process::exit(1);
}
