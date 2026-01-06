use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn echo_hello() {
    let mut cmd = std::process::Command::new(cargo::cargo_bin!("echo"));
    cmd.arg("Hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello\n"));
}

#[test]
fn echo_multiple_args() {
    let mut cmd = std::process::Command::new(cargo::cargo_bin!("echo"));
    cmd.arg("Hello")
        .arg("World")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World\n"));
}

#[test]
fn echo_no_newline() {
    let mut cmd = std::process::Command::new(cargo::cargo_bin!("echo"));
    cmd.arg("-n")
        .arg("Hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"));
}

#[test]
fn echo_no_newline_multiple_args() {
    let mut cmd = std::process::Command::new(cargo::cargo_bin!("echo"));
    cmd.arg("-n")
        .arg("Hello")
        .arg("World")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}
