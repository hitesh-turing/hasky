mod common;

use common::get_cmd;
use assert_cmd::prelude::*;
use predicates::prelude::*;

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
fn test_blake3_case_insensitive() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("BLAKE3") // uppercase
        .arg("--text")
        .arg("abc");
    cmd.assert().success().stdout(predicate::str::contains(
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
    ));
}

#[test]
fn test_unsupported_algorithm() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("invalid_algo")
        .arg("--text")
        .arg("hello");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported algorithm"));
}

