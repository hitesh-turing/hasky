use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufReader, Read};
use std::process::Command;

pub fn get_cmd() -> Command {
    let bin_path = assert_cmd::cargo::cargo_bin!("hashy");
    Command::new(bin_path)
}

/// Compute SHA-256 hash of a file using chunked reading (same as main implementation)
#[allow(dead_code)]
pub fn compute_file_sha256(file_path: &std::path::Path) -> String {
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
