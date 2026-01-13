use clap::{Arg, ArgMatches, Command};
use std::fs;
use std::io::{self, Read};

pub fn cmd() -> Command {
    Command::new("cat")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Files to concatenate")
                .num_args(0..),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number the output lines")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn run(matches: &ArgMatches) {
    let files: Vec<&str> = matches
        .get_many::<String>("files")
        .map(|vals| {
            vals.collect::<Vec<_>>()
                .iter()
                .map(|s| s.as_str())
                .collect()
        })
        .unwrap_or_default();

    let number = matches.get_flag("number");

    if files.is_empty() {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        print_with_numbers(&buffer, number);
    } else {
        for file in files {
            match fs::read_to_string(file) {
                Ok(content) => print_with_numbers(&content, number),
                Err(e) => eprintln!("cat: {}: {}", file, e),
            }
        }
    }
}

fn print_with_numbers(content: &str, number: bool) {
    if number {
        for (line_num, line) in content.lines().enumerate() {
            println!("{} {}", line_num + 1, line);
        }
    } else {
        print!("{}", content);
    }
}

pub struct Handler;
impl super::CommandHandler for Handler {
    fn run(&self, matches: &ArgMatches) {
        run(matches);
    }
}
