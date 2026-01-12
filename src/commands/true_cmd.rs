use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("true").about("Rust true")
}

pub fn run(_matches: &ArgMatches) {
    // Do nothing - exit successfully
}

pub struct Handler;
impl super::CommandHandler for Handler {
    fn run(&self, matches: &ArgMatches) {
        run(matches);
    }
}
