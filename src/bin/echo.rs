use clap::{Arg, Command};

fn main() {
    let m = Command::new("cli-rust > echo")
        .author("yamaga-shu, s.yamaga.0318@gmail.com")
        .version("0.0.1")
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
        .get_matches();

    let texts = m.get_many::<String>("text").unwrap().map(|s| s.as_str());
    let output = texts.collect::<Vec<_>>().join(" ");

    print!("{}{}", output, if m.get_flag("n") { "" } else { "\n" });
}
