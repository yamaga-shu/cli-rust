pub mod cat;
pub mod echo;
pub mod false_cmd;
pub mod true_cmd;

use clap::{ArgMatches, Command};

pub trait CommandHandler {
    fn run(&self, matches: &ArgMatches);
}

pub fn create_app() -> Command {
    Command::new("cli-rust")
        .author("yamaga-shu, s.yamaga.0318@gmail.com")
        .version("0.0.1")
        .about("Rust CLI tools")
        .subcommand(echo::cmd())
        .subcommand(cat::cmd())
        .subcommand(true_cmd::cmd())
        .subcommand(false_cmd::cmd())
}

pub fn get_commands() -> Vec<(&'static str, Box<dyn CommandHandler>)> {
    vec![
        ("echo", Box::new(echo::Handler)),
        ("cat", Box::new(cat::Handler)),
        ("true", Box::new(true_cmd::Handler)),
        ("false", Box::new(false_cmd::Handler)),
    ]
}
