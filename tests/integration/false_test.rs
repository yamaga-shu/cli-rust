use assert_cmd::cargo;
use assert_cmd::prelude::*;

#[test]
fn false_cmd() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("false").assert().failure();
}
