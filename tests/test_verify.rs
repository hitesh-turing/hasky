mod common;

use assert_cmd::prelude::*;
use common::{compute_file_sha256, get_cmd};
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_verify_ok_sha256sum_style() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("data.txt");
    fs::write(&file_path, "hello").expect("Failed to write test file");

    let digest = compute_file_sha256(&file_path);
    let manifest_path = temp_dir.path().join("checksums.txt");
    // sha256sum-style: <DIGEST>  <PATH>
    fs::write(
        &manifest_path,
        format!(
            "{}  {}\n",
            digest,
            file_path.file_name().unwrap().to_string_lossy()
        ),
    )
    .expect("Failed to write manifest");

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path())
        .arg("verify")
        .arg(manifest_path.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("data.txt: OK"))
        .stdout(predicate::str::contains("All files verified successfully."));
}

#[test]
fn test_verify_mismatch() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("file.txt");
    fs::write(&file_path, "abc").expect("Failed to write test file");

    let wrong_digest = "0".repeat(64); // invalid digest value but correct length
    let manifest_path = temp_dir.path().join("checksums.txt");
    fs::write(
        &manifest_path,
        format!(
            "{}  {}\n",
            wrong_digest,
            file_path.file_name().unwrap().to_string_lossy()
        ),
    )
    .expect("Failed to write manifest");

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path())
        .arg("verify")
        .arg(manifest_path.as_os_str());

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("file.txt: FAILED (mismatch)"));
}

#[test]
fn test_verify_missing_file_and_continue() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let present = temp_dir.path().join("present.txt");
    fs::write(&present, "xyz").expect("Failed to write test file");

    let digest_present = compute_file_sha256(&present);
    let manifest_path = temp_dir.path().join("checksums.txt");

    // Include one correct and one missing
    let manifest_content = format!(
        "{}  present.txt\n{}  missing.txt\n",
        digest_present, digest_present
    );
    fs::write(&manifest_path, manifest_content).expect("Failed to write manifest");

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path())
        .arg("verify")
        .arg("--continue-on-error")
        .arg(manifest_path.as_os_str());

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("present.txt: OK"))
        .stdout(predicate::str::contains(
            "missing.txt: FAILED (Failed to open file",
        ))
        .stdout(predicate::str::contains("Summary: 1 succeeded, 1 failed"));
}

#[test]
fn test_verify_verbose_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("v.txt");
    fs::write(&file_path, "verbose").expect("Failed to write test file");

    let digest = compute_file_sha256(&file_path);
    let manifest_path = temp_dir.path().join("checksums.txt");
    fs::write(
        &manifest_path,
        format!(
            "{}  {}\n",
            digest,
            file_path.file_name().unwrap().to_string_lossy()
        ),
    )
    .expect("Failed to write manifest");

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path())
        .arg("--verbose")
        .arg("verify")
        .arg(manifest_path.as_os_str());

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Using algorithm"))
        .stderr(predicate::str::contains("Verifying file: v.txt"))
        .stdout(predicate::str::contains("v.txt: OK"));
}
