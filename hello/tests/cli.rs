use assert_cmd::cargo;
use assert_cmd::prelude::*;

use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new(cargo::cargo_bin!("hello"));

    cmd.assert().success();
}
