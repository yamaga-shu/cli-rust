use assert_cmd::cargo;
use assert_cmd::prelude::*;

#[test]
fn cat_no_args() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat")
        .assert()
        .success();
}
