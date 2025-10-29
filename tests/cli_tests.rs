use assert_cmd::prelude::*;
use predicates::prelude::*;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufReader, Read, Write};
use std::process::Command;
use tempfile::TempDir;

fn get_cmd() -> Command {
    let bin_path = assert_cmd::cargo::cargo_bin!("hashy");
    Command::new(bin_path)
}

/// Compute SHA-256 hash of a file using chunked reading (same as main implementation)
fn compute_file_sha256(file_path: &std::path::Path) -> String {
    const CHUNK_SIZE: usize = 64 * 1024; // 64 KiB

    let file = fs::File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read from file");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    hex::encode(hasher.finalize())
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

// Test file hashing with different algorithms
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

// File hashing tests
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
