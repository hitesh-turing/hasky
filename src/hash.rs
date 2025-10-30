use crate::algorithm::Algorithm;
use anyhow::{Context, Result};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read};

/// Hash data using the specified algorithm
pub fn hash_data(algorithm: Algorithm, data: &[u8]) -> String {
    match algorithm {
        Algorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Blake3 => {
            let hash = blake3::hash(data);
            hex::encode(hash.as_bytes())
        }
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    }
}

/// Hash a file using the specified algorithm by reading it in chunks (64 KiB).
/// This avoids loading the entire file into memory.
pub fn hash_file(algorithm: Algorithm, file_path: &str) -> Result<String> {
    const CHUNK_SIZE: usize = 64 * 1024; // 64 KiB

    let file =
        File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;

    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; CHUNK_SIZE];

    match algorithm {
        Algorithm::Sha1 => {
            let mut hasher = Sha1::new();
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .with_context(|| format!("Failed to read from file: {}", file_path))?;

                if bytes_read == 0 {
                    break;
                }

                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .with_context(|| format!("Failed to read from file: {}", file_path))?;

                if bytes_read == 0 {
                    break;
                }

                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .with_context(|| format!("Failed to read from file: {}", file_path))?;

                if bytes_read == 0 {
                    break;
                }

                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        Algorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .with_context(|| format!("Failed to read from file: {}", file_path))?;

                if bytes_read == 0 {
                    break;
                }

                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hex::encode(hasher.finalize().as_bytes()))
        }
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .with_context(|| format!("Failed to read from file: {}", file_path))?;

                if bytes_read == 0 {
                    break;
                }

                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
    }
}
