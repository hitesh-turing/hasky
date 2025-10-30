mod common;

use common::get_cmd;
use assert_cmd::prelude::*;
use predicates::prelude::*;

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
fn test_help_shows_algorithms() {
    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sha256"))
        .stdout(predicate::str::contains("sha512"))
        .stdout(predicate::str::contains("blake3"))
        .stdout(predicate::str::contains("--allow-insecure"));
}

