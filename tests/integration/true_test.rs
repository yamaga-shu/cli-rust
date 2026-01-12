use assert_cmd::cargo;
use assert_cmd::prelude::*;

#[test]
fn true_cmd() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("true").assert().success();
}
