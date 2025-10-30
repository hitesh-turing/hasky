mod common;

use assert_cmd::prelude::*;
use common::get_cmd;
use predicates::prelude::*;
use serde_json;
use std::io::Write;
use std::process::Stdio;

#[test]
fn test_format_hex() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("hex");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert_eq!(
        stdout.trim(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn test_format_uppercase_hex() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("hex")
        .arg("--uppercase");
    cmd.assert().success().stdout(predicate::str::contains(
        "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD",
    ));
}

#[test]
fn test_format_base64() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("base64");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert_eq!(
        stdout.trim(),
        "ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0="
    );
}

#[test]
fn test_format_json() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc")
        .arg("--json");
    cmd.assert().success();

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Verify required fields
    assert_eq!(json["algo"], "sha256");
    assert_eq!(json["source"], "text");
    assert_eq!(
        json["digest"],
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(json["bytes"], 3);
}

#[test]
fn test_format_json_stdin() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"abc").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Verify required fields
    assert_eq!(json["algo"], "sha256");
    assert_eq!(json["source"], "stdin");
    assert_eq!(
        json["digest"],
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(json["bytes"], 3);
}

#[test]
fn test_format_json_file() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"abc").unwrap();
    let path = file.path().to_str().unwrap();

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--file")
        .arg(path)
        .arg("--json");
    cmd.assert().success();

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Verify required fields
    assert_eq!(json["algo"], "sha256");
    assert_eq!(json["source"], "file");
    assert_eq!(
        json["digest"],
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(json["bytes"], 3);
}

#[test]
fn test_format_default_behavior() {
    // Test that default behavior still works (multi-line output)
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Algorithm: sha256"))
        .stdout(predicate::str::contains("Text: abc"))
        .stdout(predicate::str::contains(
            "Digest: ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ));
}

#[test]
fn test_format_conflicts_json() {
    // Test that --format and --json conflict
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("hex")
        .arg("--json");
    cmd.assert().failure();
}

#[test]
fn test_format_invalid() {
    // Test invalid format
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_format_raw() {
    // Test raw bytes output
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("raw");
    let output = cmd.output().unwrap();
    assert!(output.status.success());

    // SHA-256 of "abc" is 32 bytes
    assert_eq!(output.stdout.len(), 32);

    // Verify it matches the expected raw bytes
    let expected_hex = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let expected_bytes = hex::decode(expected_hex).unwrap();
    assert_eq!(output.stdout, expected_bytes);
}

#[test]
fn test_format_hex_no_uppercase_flag() {
    // Test that hex format without uppercase flag still outputs lowercase
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("abc")
        .arg("--format")
        .arg("hex");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    // Should be lowercase by default
    assert_eq!(
        stdout.trim(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn test_format_json_with_stdin_empty() {
    // Test JSON output with empty stdin
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Verify required fields
    assert_eq!(json["algo"], "sha256");
    assert_eq!(json["source"], "stdin");
    assert_eq!(
        json["digest"],
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(json["bytes"], 0);
}
