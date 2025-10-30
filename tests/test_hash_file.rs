mod common;

use common::{compute_file_sha256, get_cmd};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_hash_file_hello() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("hello.txt");
    fs::write(&file_path, "hello").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--file").arg(file_path.as_os_str());

    cmd.assert().success().stdout(predicate::str::contains(
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
    ));
}

#[test]
fn test_hash_file_empty() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("empty.txt");
    fs::write(&file_path, "").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--file").arg(file_path.as_os_str());

    cmd.assert().success().stdout(predicate::str::contains(
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ));
}

#[test]
fn test_hash_file_sha512() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "abc").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha512")
        .arg("--file")
        .arg(file_path.as_os_str());

    cmd.assert().success().stdout(predicate::str::contains(
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
    ));
}

#[test]
fn test_hash_file_blake3() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "abc").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("blake3")
        .arg("--file")
        .arg(file_path.as_os_str());

    cmd.assert().success().stdout(predicate::str::contains(
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
    ));
}

#[test]
fn test_hash_file_large_chunked() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("large.txt");

    // Create a file larger than 64 KiB to test chunking (~100 KiB)
    let mut file = fs::File::create(&file_path).expect("Failed to create test file");
    let data = b"The quick brown fox jumps over the lazy dog\n";
    // Write ~100 KiB of data (about 1.6x the chunk size)
    for _ in 0..2560 {
        file.write_all(data).expect("Failed to write to test file");
    }
    drop(file);

    // Compute expected hash using Rust implementation
    let expected_hash = compute_file_sha256(&file_path);

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--file").arg(file_path.as_os_str());

    let cmd_output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&cmd_output.stdout);

    assert!(
        stdout.trim().contains(&expected_hash),
        "Hash mismatch. Expected: {}, Got: {}",
        expected_hash,
        stdout.trim()
    );
    assert!(cmd_output.status.success());
}

#[test]
fn test_hash_file_matches_sha256sum() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    let content = "The quick brown fox jumps over the lazy dog";
    fs::write(&file_path, content).expect("Failed to write test file");

    // Compute expected hash using Rust implementation (same algorithm as sha256sum)
    let expected_hash = compute_file_sha256(&file_path);

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--file").arg(file_path.as_os_str());

    let cmd_output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&cmd_output.stdout);

    assert!(
        stdout.trim().contains(&expected_hash),
        "Hash mismatch. Expected: {}, Got: {}",
        expected_hash,
        stdout.trim()
    );
    assert!(cmd_output.status.success());
}

#[test]
fn test_hash_file_nonexistent() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--file")
        .arg("nonexistent_file_12345.txt");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to hash file"));
}

#[test]
fn test_hash_file_verbose() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("verbose_test.txt");
    fs::write(&file_path, "test").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("--verbose")
        .arg("hash")
        .arg("--file")
        .arg(file_path.as_os_str());

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Hashing file"));
}

#[test]
fn test_hash_file_quiet() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("quiet_test.txt");
    fs::write(&file_path, "test").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("--quiet")
        .arg("hash")
        .arg("--file")
        .arg(file_path.as_os_str());

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).trim().is_empty());
}

