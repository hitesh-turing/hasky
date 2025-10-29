use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn get_cmd() -> Command {
    let bin_path = assert_cmd::cargo::cargo_bin!("hashy");
    Command::new(bin_path)
}

#[test]
fn test_help_command() {
    let mut cmd = get_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A fast, flexible CLI for hashing"));
}

#[test]
fn test_version_command() {
    let mut cmd = get_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hashy"));
}

#[test]
fn test_hash_help_command() {
    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Compute hash of input"))
        .stdout(predicate::str::contains("--algo"))
        .stdout(predicate::str::contains("--text"))
        .stdout(predicate::str::contains("--file"));
}

#[test]
fn test_hash_with_text() {
    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--text").arg("hello");
    cmd.assert().success();
}

#[test]
fn test_hash_with_custom_algo() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("blake3")
        .arg("--text")
        .arg("hello");
    cmd.assert().success();
}

#[test]
fn test_verbose_flag() {
    let mut cmd = get_cmd();
    cmd.arg("--verbose").arg("hash").arg("--text").arg("hello");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Using algorithm"));
}

#[test]
fn test_quiet_flag() {
    let mut cmd = get_cmd();
    cmd.arg("--quiet").arg("hash").arg("--text").arg("hello");
    cmd.assert().success();
}

#[test]
fn test_conflicting_text_and_file() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--text")
        .arg("hello")
        .arg("--file")
        .arg("test.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_conflicting_quiet_and_verbose() {
    let mut cmd = get_cmd();
    cmd.arg("--quiet").arg("--verbose").arg("hash");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}
