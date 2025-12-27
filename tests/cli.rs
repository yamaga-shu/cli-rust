use assert_cmd::cargo;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new(cargo::cargo_bin!("cli-rust"));

    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::new(cargo::cargo_bin!("true"));

    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::new(cargo::cargo_bin!("false"));

    cmd.assert().failure();
}
