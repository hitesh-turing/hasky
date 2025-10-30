mod common;

use assert_cmd::prelude::*;
use common::get_cmd;
use predicates::prelude::*;
use std::io::Write;
use std::process::Stdio;

#[test]
fn test_hash_from_pipe_sha256() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"hello").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",));
}

#[test]
fn test_hash_from_pipe_without_algo() {
    // Default algorithm should be sha256
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"hello").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",));
}

#[test]
fn test_hash_from_pipe_empty_string() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",));
}

#[test]
fn test_hash_from_pipe_sha512() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha512")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"abc").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
    ));
}

#[test]
fn test_hash_from_pipe_blake3() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("blake3")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"abc").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",));
}

#[test]
fn test_hash_from_file_dash_placeholder() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--file")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"hello").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",));
}

#[test]
fn test_hash_from_file_dash_placeholder_empty() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--file")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",));
}

#[test]
fn test_hash_interactive_tty_error() {
    // Note: In test environment, stdin is not a TTY, so this will read empty input
    // instead of showing an error. The actual behavior (TTY detection) can only be
    // tested manually in an interactive terminal.
    // In test environment, non-TTY stdin with no data results in empty hash
    let mut cmd = get_cmd();
    cmd.arg("hash");

    // In automated tests, stdin is not a TTY, so it reads empty input
    // The error only appears when running interactively (when stdin IS a TTY)
    cmd.assert().success().stdout(predicate::str::contains(
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", // Empty string hash
    ));
}

#[test]
fn test_hash_stdin_verbose() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("--verbose")
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"hello").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stderr.contains("Hashing from STDIN"));
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",));
}

#[test]
fn test_hash_stdin_quiet() {
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("--quiet")
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"hello").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).trim().is_empty());
}

#[test]
fn test_hash_stdin_large_input_chunked() {
    // Test that large inputs are handled correctly with chunked reading
    let large_data = "The quick brown fox jumps over the lazy dog\n".repeat(2560); // ~100 KiB

    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(large_data.as_bytes())
        .unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
}

#[test]
fn test_hash_stdin_no_label_in_output() {
    // STDIN should not show "Stdin:" label in output, just algorithm and digest
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(b"hello").unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Algorithm:"));
    assert!(stdout.contains("Digest:"));
    assert!(!stdout.contains("Stdin:"));
    assert!(output.status.success());
}

#[test]
fn test_hash_text_overrides_stdin() {
    // If --text is provided, it should override STDIN
    let mut cmd = get_cmd();
    let mut child = cmd
        .arg("hash")
        .arg("--algo")
        .arg("sha256")
        .arg("--text")
        .arg("hello")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"this should be ignored")
        .unwrap();
    child.stdin.as_mut().unwrap().flush().unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Text: hello"));
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",));
}
