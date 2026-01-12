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
        ("echo", Box::new(EchoHandler)),
        ("cat", Box::new(CatHandler)),
        ("true", Box::new(TrueHandler)),
        ("false", Box::new(FalseHandler)),
    ]
}

struct EchoHandler;
impl CommandHandler for EchoHandler {
    fn run(&self, matches: &ArgMatches) {
        echo::run(matches);
    }
}

struct CatHandler;
impl CommandHandler for CatHandler {
    fn run(&self, matches: &ArgMatches) {
        cat::run(matches);
    }
}

struct TrueHandler;
impl CommandHandler for TrueHandler {
    fn run(&self, matches: &ArgMatches) {
        true_cmd::run(matches);
    }
}

struct FalseHandler;
impl CommandHandler for FalseHandler {
    fn run(&self, matches: &ArgMatches) {
        false_cmd::run(matches);
    }
}
