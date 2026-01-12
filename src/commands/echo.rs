use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("echo")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("n")
                .short('n')
                .help("Do not print newline")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
}

pub fn run(matches: &ArgMatches) {
    let texts: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str())
        .collect();
    let no_newline = matches.get_flag("n");
    print_echo(texts, no_newline);
}

pub fn print_echo(text_args: Vec<&str>, no_newline: bool) {
    let output = text_args.join(" ");
    print!("{}{}", output, if no_newline { "" } else { "\n" });
}
