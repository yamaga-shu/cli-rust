use clap::Command;

fn main() {
    let _m = Command::new("cli-rust")
        .author("yamaga-shu, s.yamaga.0318@gmail.com")
        .version("0.0.1")
        .about("Rust echo")
        .get_matches();
}
