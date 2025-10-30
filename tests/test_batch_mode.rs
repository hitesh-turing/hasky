mod common;

use assert_cmd::prelude::*;
use common::get_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_batch_hash_multiple_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");
    fs::write(&file2, "world").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg(file1.as_os_str())
        .arg(file2.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ))
        .stdout(predicate::str::contains(
            "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7",
        ));
}

#[test]
fn test_batch_hash_with_missing_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let missing_file = temp_dir.path().join("missing.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg(file1.as_os_str())
        .arg(missing_file.as_os_str());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to hash file"));
}

#[test]
fn test_batch_hash_continue_on_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let missing_file = temp_dir.path().join("missing.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");
    fs::write(&file2, "world").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--continue-on-error")
        .arg(file1.as_os_str())
        .arg(missing_file.as_os_str())
        .arg(file2.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ))
        .stdout(predicate::str::contains(
            "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7",
        ))
        .stdout(predicate::str::contains("ERROR"))
        .stdout(predicate::str::contains("Summary: 2 succeeded, 1 failed"));
}

#[test]
fn test_batch_hash_json_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");
    fs::write(&file2, "world").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--json")
        .arg(file1.as_os_str())
        .arg(file2.as_os_str());

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"algo\":\"sha256\""));
    assert!(stdout.contains("\"results\":"));
    assert!(stdout.contains("\"file_path\""));
}

#[test]
fn test_batch_hash_json_with_errors() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let missing_file = temp_dir.path().join("missing.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--json")
        .arg("--continue-on-error")
        .arg(file1.as_os_str())
        .arg(missing_file.as_os_str());

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"success\":true"));
    assert!(stdout.contains("\"success\":false"));
    assert!(stdout.contains("\"summary\""));
}

#[test]
fn test_batch_hash_verbose() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");
    fs::write(&file2, "world").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("--verbose")
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg(file1.as_os_str())
        .arg(file2.as_os_str());

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Using algorithm"))
        .stderr(predicate::str::contains("Hashing 2 files"))
        .stderr(predicate::str::contains("Hashing file"));
}

#[test]
fn test_batch_hash_quiet() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");
    fs::write(&file2, "world").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("--quiet")
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg(file1.as_os_str())
        .arg(file2.as_os_str());

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).trim().is_empty());
}

#[test]
fn test_batch_hash_conflicts_with_text() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--text")
        .arg("hello")
        .arg(file1.as_os_str());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_batch_hash_conflicts_with_single_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");
    fs::write(&file2, "world").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--file")
        .arg(file1.as_os_str())
        .arg(file2.as_os_str());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_batch_hash_format_options() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--format")
        .arg("base64")
        .arg(file1.as_os_str());

    cmd.assert().success().stdout(predicate::str::contains(
        "LPJNul+wow4m6DsqxbninhsWHlwfp0JecwQzYpOLmCQ=",
    ));
}

#[test]
fn test_batch_hash_uppercase() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");

    fs::write(&file1, "hello").expect("Failed to write test file");

    let mut cmd = get_cmd();
    cmd.arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--uppercase")
        .arg(file1.as_os_str());

    cmd.assert().success().stdout(predicate::str::contains(
        "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824",
    ));
}

#[test]
fn test_batch_hash_order_preservation() {
    // Test that parallel processing preserves the order of files
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let files: Vec<_> = (1..=10)
        .map(|i| {
            let file = temp_dir.path().join(format!("file{}.txt", i));
            fs::write(&file, format!("content{}", i)).expect("Failed to write test file");
            file
        })
        .collect();

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--algo").arg("sha256");
    for file in &files {
        cmd.arg(file.as_os_str());
    }

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();

    // Verify that files appear in the same order as input
    for (i, line) in lines.iter().enumerate() {
        let file_name = format!("file{}.txt", i + 1);
        assert!(
            line.contains(&file_name),
            "Expected file {} at position {}, got: {}",
            file_name,
            i,
            line
        );
    }
}

#[test]
fn test_batch_hash_many_files() {
    // Test parallel processing with many files
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let mut files = Vec::new();
    for i in 1..=20 {
        let file = temp_dir.path().join(format!("file{}.txt", i));
        fs::write(&file, format!("data{}", i)).expect("Failed to write test file");
        files.push(file);
    }

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--algo").arg("sha256");
    for file in &files {
        cmd.arg(file.as_os_str());
    }

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 20, "Expected 20 output lines");

    // Verify all files were hashed
    for i in 1..=20 {
        let file_name = format!("file{}.txt", i);
        assert!(
            stdout.contains(&file_name),
            "Expected output to contain {}",
            file_name
        );
    }
}

#[test]
fn test_batch_hash_all_algorithms() {
    // Test parallel processing works with all algorithms
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    let file3 = temp_dir.path().join("file3.txt");

    fs::write(&file1, "test1").expect("Failed to write test file");
    fs::write(&file2, "test2").expect("Failed to write test file");
    fs::write(&file3, "test3").expect("Failed to write test file");

    for algo in &["sha256", "sha512", "blake3"] {
        let mut cmd = get_cmd();
        cmd.arg("hash")
            .arg("--algo")
            .arg(algo)
            .arg(file1.as_os_str())
            .arg(file2.as_os_str())
            .arg(file3.as_os_str());

        let output = cmd.output().expect("Failed to execute command");
        assert!(
            output.status.success(),
            "Failed to hash files with algorithm {}",
            algo
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("file1.txt"));
        assert!(stdout.contains("file2.txt"));
        assert!(stdout.contains("file3.txt"));
    }
}

#[test]
fn test_batch_hash_json_order_preservation() {
    // Test that JSON output preserves order with parallel processing
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let files: Vec<_> = (1..=5)
        .map(|i| {
            let file = temp_dir.path().join(format!("file{}.txt", i));
            fs::write(&file, format!("data{}", i)).expect("Failed to write test file");
            file
        })
        .collect();

    let mut cmd = get_cmd();
    cmd.arg("hash").arg("--algo").arg("sha256").arg("--json");
    for file in &files {
        cmd.arg(file.as_os_str());
    }

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("Failed to parse JSON output");

    let results = json["results"]
        .as_array()
        .expect("Expected 'results' to be an array");

    assert_eq!(results.len(), 5);

    // Verify order is preserved
    for (i, result) in results.iter().enumerate() {
        let expected_file = format!("file{}.txt", i + 1);
        let actual_file = result["file_path"]
            .as_str()
            .expect("Expected 'file_path' to be a string");
        assert!(
            actual_file.contains(&expected_file),
            "Expected file {} at position {}, got {}",
            expected_file,
            i,
            actual_file
        );
    }
}
