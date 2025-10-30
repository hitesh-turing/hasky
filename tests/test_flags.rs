mod common;

use assert_cmd::prelude::*;
use common::get_cmd;
use predicates::prelude::*;

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
