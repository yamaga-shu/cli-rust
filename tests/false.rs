use assert_cmd::cargo;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn false_not_ok() {
    let mut cmd = Command::new(cargo::cargo_bin!("false"));

    cmd.assert().failure();
}
