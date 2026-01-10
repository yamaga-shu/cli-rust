use clap::Command;

fn main() {
    let m = Command::new("cli-rust > cat")
        .author("yamaga-shu, s.yamaga.0318@gmail.com")
        .version("0.0.1")
        .about("Rust cat")
        .get_matches();
}
