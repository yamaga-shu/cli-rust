use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn echo_hello() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("echo")
        .arg("Hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello\n"));
}

#[test]
fn echo_multiple_args() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("echo")
        .arg("Hello")
        .arg("World")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World\n"));
}

#[test]
fn echo_no_newline() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("echo")
        .arg("-n")
        .arg("Hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"))
        .stdout(predicate::str::ends_with("Hello"));
}
