mod common;

use assert_cmd::prelude::*;
use common::get_cmd;
use predicates::prelude::*;

#[test]
fn test_hash_with_text() {
    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--text").arg("hello");
    cmd.assert().success();
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

// SHA-512 test vector tests
#[test]
fn test_sha512_abc() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha512")
        .arg("--text")
        .arg("abc");
    cmd.assert().success().stdout(predicate::str::contains(
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
    ));
}

#[test]
fn test_sha512_empty_string() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha512")
        .arg("--text")
        .arg("");
    cmd.assert().success().stdout(predicate::str::contains(
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
    ));
}

// BLAKE3 test vector tests
#[test]
fn test_blake3_abc() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("blake3")
        .arg("--text")
        .arg("abc");
    cmd.assert().success().stdout(predicate::str::contains(
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
    ));
}

#[test]
fn test_blake3_empty_string() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("blake3")
        .arg("--text")
        .arg("");
    // BLAKE3 of empty string
    cmd.assert().success().stdout(predicate::str::contains(
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ));
}
