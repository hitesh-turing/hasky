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
    // Test that explicitly specifying sha256 works
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("hello");
    cmd.assert().success().stdout(predicate::str::contains(
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
    ));
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

// SHA-256 test vector tests
#[test]
fn test_sha256_abc() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc");
    cmd.assert().success().stdout(predicate::str::contains(
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    ));
}

#[test]
fn test_sha256_empty_string() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("");
    cmd.assert().success().stdout(predicate::str::contains(
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ));
}

#[test]
fn test_sha256_hello() {
    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--text").arg("hello"); // sha256 is default
    cmd.assert().success().stdout(predicate::str::contains(
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
    ));
}

#[test]
fn test_sha256_longer_text() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("The quick brown fox jumps over the lazy dog");
    cmd.assert().success().stdout(predicate::str::contains(
        "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592",
    ));
}

#[test]
fn test_sha256_case_insensitive() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("SHA256") // uppercase
        .arg("--text")
        .arg("abc");
    cmd.assert().success().stdout(predicate::str::contains(
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    ));
}

#[test]
fn test_unsupported_algorithm() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("md5")
        .arg("--text")
        .arg("hello");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported algorithm"));
}
