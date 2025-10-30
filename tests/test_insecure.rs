mod common;

use common::get_cmd;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_insecure_algorithm_requires_flag() {
    // Test that md5 requires --allow-insecure
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("md5")
        .arg("--text")
        .arg("hello");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Insecure algorithm"))
        .stderr(predicate::str::contains("--allow-insecure"));

    // Test that sha1 requires --allow-insecure
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha1")
        .arg("--text")
        .arg("hello");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Insecure algorithm"))
        .stderr(predicate::str::contains("--allow-insecure"));
}

// SHA-1 test vector tests (requires --allow-insecure)
#[test]
fn test_sha1_abc_with_allow_insecure() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha1")
        .arg("--allow-insecure")
        .arg("--text")
        .arg("abc");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "a9993e364706816aba3e25717850c26c9cd0d89d",
        ))
        .stderr(predicate::str::contains("WARNING"));
}

#[test]
fn test_sha1_empty_string_with_allow_insecure() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha1")
        .arg("--allow-insecure")
        .arg("--text")
        .arg("");
    cmd.assert().success().stdout(predicate::str::contains(
        "da39a3ee5e6b4b0d3255bfef95601890afd80709",
    ));
}

#[test]
fn test_hash_file_sha1_with_allow_insecure() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "abc").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha1")
        .arg("--allow-insecure")
        .arg("--file")
        .arg(file_path.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "a9993e364706816aba3e25717850c26c9cd0d89d",
        ))
        .stderr(predicate::str::contains("WARNING"));
}

// MD5 test vector tests (requires --allow-insecure)
#[test]
fn test_md5_abc_with_allow_insecure() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("md5")
        .arg("--allow-insecure")
        .arg("--text")
        .arg("abc");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("900150983cd24fb0d6963f7d28e17f72"))
        .stderr(predicate::str::contains("WARNING"));
}

#[test]
fn test_md5_empty_string_with_allow_insecure() {
    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("md5")
        .arg("--allow-insecure")
        .arg("--text")
        .arg("");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("d41d8cd98f00b204e9800998ecf8427e"));
}

#[test]
fn test_hash_file_md5_with_allow_insecure() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "abc").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("md5")
        .arg("--allow-insecure")
        .arg("--file")
        .arg(file_path.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("900150983cd24fb0d6963f7d28e17f72"))
        .stderr(predicate::str::contains("WARNING"));
}

