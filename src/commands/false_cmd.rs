use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("false").about("Rust false")
}

pub fn run(_matches: &ArgMatches) {
    std::process::exit(1);
}

pub struct Handler;
impl super::CommandHandler for Handler {
    fn run(&self, matches: &ArgMatches) {
        run(matches);
    }
}
