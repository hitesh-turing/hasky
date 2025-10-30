use crate::algorithm::Algorithm;
use anyhow::{Context, Result};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fs::File;
use std::io::{stdin, BufReader, Read};

/// Hash data using the specified algorithm, returns raw bytes
pub fn hash_data(algorithm: Algorithm, data: &[u8]) -> Vec<u8> {
    match algorithm {
        Algorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        Algorithm::Blake3 => {
            let hash = blake3::hash(data);
            hash.as_bytes().to_vec()
        }
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
    }
}

/// Hash a file using the specified algorithm by reading it in chunks (64 KiB).
/// This avoids loading the entire file into memory.
/// Returns raw bytes of the hash.
pub fn hash_file(algorithm: Algorithm, file_path: &str) -> Result<Vec<u8>> {
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
            Ok(hasher.finalize().to_vec())
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
            Ok(hasher.finalize().to_vec())
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
            Ok(hasher.finalize().to_vec())
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
            Ok(hasher.finalize().as_bytes().to_vec())
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
            Ok(hasher.finalize().to_vec())
        }
    }
}

/// Hash STDIN using the specified algorithm by reading it in chunks (64 KiB).
/// This avoids loading the entire input into memory.
/// Returns (hash_bytes, input_size)
pub fn hash_stdin(algorithm: Algorithm) -> Result<(Vec<u8>, usize)> {
    const CHUNK_SIZE: usize = 64 * 1024; // 64 KiB

    let stdin_handle = stdin();
    let mut reader = BufReader::new(stdin_handle.lock());
    let mut buffer = vec![0u8; CHUNK_SIZE];

    match algorithm {
        Algorithm::Sha1 => {
            let mut hasher = Sha1::new();
            let mut total_bytes = 0;
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .context("Failed to read from STDIN")?;

                if bytes_read == 0 {
                    break;
                }

                total_bytes += bytes_read;
                hasher.update(&buffer[..bytes_read]);
            }
            Ok((hasher.finalize().to_vec(), total_bytes))
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            let mut total_bytes = 0;
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .context("Failed to read from STDIN")?;

                if bytes_read == 0 {
                    break;
                }

                total_bytes += bytes_read;
                hasher.update(&buffer[..bytes_read]);
            }
            Ok((hasher.finalize().to_vec(), total_bytes))
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            let mut total_bytes = 0;
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .context("Failed to read from STDIN")?;

                if bytes_read == 0 {
                    break;
                }

                total_bytes += bytes_read;
                hasher.update(&buffer[..bytes_read]);
            }
            Ok((hasher.finalize().to_vec(), total_bytes))
        }
        Algorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            let mut total_bytes = 0;
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .context("Failed to read from STDIN")?;

                if bytes_read == 0 {
                    break;
                }

                total_bytes += bytes_read;
                hasher.update(&buffer[..bytes_read]);
            }
            Ok((hasher.finalize().as_bytes().to_vec(), total_bytes))
        }
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            let mut total_bytes = 0;
            loop {
                let bytes_read = reader
                    .read(&mut buffer)
                    .context("Failed to read from STDIN")?;

                if bytes_read == 0 {
                    break;
                }

                total_bytes += bytes_read;
                hasher.update(&buffer[..bytes_read]);
            }
            Ok((hasher.finalize().to_vec(), total_bytes))
        }
    }
}
