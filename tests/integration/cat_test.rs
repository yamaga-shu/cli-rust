use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::path::PathBuf;

fn get_fixture(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/inputs");
    path.push(name);
    path
}

#[test]
fn cat_single_file() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat")
        .arg(get_fixture("fox.txt"))
        .assert()
        .success()
        .stdout(predicate::str::contains("quick brown fox"));
}

#[test]
fn cat_empty_file() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat")
        .arg(get_fixture("empty.txt"))
        .assert()
        .success()
        .stdout("");
}

#[test]
fn cat_multiple_files() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat")
        .arg(get_fixture("fox.txt"))
        .arg(get_fixture("spiders.txt"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("quick brown fox").and(predicate::str::contains("spider")),
        );
}

#[test]
fn cat_with_number() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat")
        .arg("-n")
        .arg(get_fixture("fox.txt"))
        .assert()
        .success()
        .stdout(predicate::str::contains("1 ").and(predicate::str::contains("quick brown fox")));
}

#[test]
fn cat_with_number_long_flag() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat")
        .arg("--number")
        .arg(get_fixture("fox.txt"))
        .assert()
        .success()
        .stdout(predicate::str::contains("1 ").and(predicate::str::contains("quick brown fox")));
}

#[test]
fn cat_no_args() {
    let cmd = cargo::cargo_bin!("cli-rust");
    let mut cmd = std::process::Command::new(cmd);
    cmd.arg("cat").assert().success();
}
