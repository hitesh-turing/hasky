use anyhow::{anyhow, Context, Result};
use blake3;
use clap::{Parser, Subcommand};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};

/// A fast, flexible CLI for hashing with multiple algorithms
#[derive(Parser, Debug)]
#[command(name = "hashy")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compute hash of input (text, file, or stdin)
    Hash {
        /// Hash algorithm to use
        /// 
        /// Supported algorithms: sha256 (default), sha512, blake3
        /// 
        /// Insecure algorithms (sha1, md5) require --allow-insecure flag.
        /// WARNING: SHA-1 and MD5 are cryptographically broken and should
        /// only be used for legacy compatibility or non-security purposes.
        #[arg(
            short,
            long,
            value_name = "ALGORITHM",
            default_value = "sha256",
            help = "Hash algorithm to use [possible values: sha256, sha512, blake3, sha1, md5]"
        )]
        algo: String,

        /// Allow use of insecure algorithms (SHA-1 and MD5)
        /// 
        /// WARNING: SHA-1 and MD5 are cryptographically broken and vulnerable
        /// to collision attacks. Only use these algorithms for legacy
        /// compatibility or non-security purposes.
        #[arg(long)]
        allow_insecure: bool,

        /// Text to hash directly
        #[arg(short, long, value_name = "TEXT", conflicts_with = "file")]
        text: Option<String>,

        /// File to hash
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging/verbosity
    let verbosity = if cli.quiet {
        Verbosity::Quiet
    } else if cli.verbose {
        Verbosity::Verbose
    } else {
        Verbosity::Normal
    };

    match &cli.command {
        Commands::Hash {
            algo,
            allow_insecure,
            text,
            file,
        } => {
            handle_hash(
                algo,
                *allow_insecure,
                text.as_deref(),
                file.as_deref(),
                verbosity,
            )?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// Supported hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Sha1,
    Sha256,
    Sha512,
    Blake3,
    Md5,
}

impl Algorithm {
    /// Parse algorithm from string (case-insensitive)
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "sha1" => Ok(Algorithm::Sha1),
            "sha256" => Ok(Algorithm::Sha256),
            "sha512" => Ok(Algorithm::Sha512),
            "blake3" => Ok(Algorithm::Blake3),
            "md5" => Ok(Algorithm::Md5),
            _ => Err(anyhow!("Unsupported algorithm: {}", s)),
        }
    }

    /// Check if algorithm is considered insecure
    fn is_insecure(&self) -> bool {
        matches!(self, Algorithm::Sha1 | Algorithm::Md5)
    }

    /// Get display name for the algorithm
    fn name(&self) -> &'static str {
        match self {
            Algorithm::Sha1 => "sha1",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha512 => "sha512",
            Algorithm::Blake3 => "blake3",
            Algorithm::Md5 => "md5",
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Hash data using the specified algorithm
fn hash_data(algorithm: Algorithm, data: &[u8]) -> String {
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
fn hash_file(algorithm: Algorithm, file_path: &str) -> Result<String> {
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

fn handle_hash(
    algo_str: &str,
    allow_insecure: bool,
    text: Option<&str>,
    file: Option<&str>,
    verbosity: Verbosity,
) -> Result<()> {
    // Parse algorithm
    let algorithm = Algorithm::from_str(algo_str)?;

    // Check security gating
    if algorithm.is_insecure() && !allow_insecure {
        eprintln!(
            "WARNING: {} is considered cryptographically insecure and vulnerable to collision attacks.",
            algorithm.name()
        );
        eprintln!("Use --allow-insecure to enable this algorithm (only for legacy compatibility or non-security purposes).");
        return Err(anyhow!(
            "Insecure algorithm '{}' requires --allow-insecure flag",
            algorithm.name()
        ));
    }

    // Warn even when allowed
    if algorithm.is_insecure() && allow_insecure && !matches!(verbosity, Verbosity::Quiet) {
        eprintln!(
            "WARNING: {} is cryptographically broken and should not be used for security purposes.",
            algorithm.name()
        );
    }

    if matches!(verbosity, Verbosity::Verbose) {
        eprintln!("Using algorithm: {}", algorithm);
        if let Some(t) = text {
            eprintln!("Hashing text: {}", t);
        } else if let Some(f) = file {
            eprintln!("Hashing file: {}", f);
        } else {
            eprintln!("Ready to hash from stdin");
        }
    }

    // Compute hash based on algorithm and input type
    let hash = if let Some(t) = text {
        hash_data(algorithm, t.as_bytes())
    } else if let Some(f) = file {
        hash_file(algorithm, f).with_context(|| format!("Failed to hash file: {}", f))?
    } else {
        // TODO: Implement stdin reading in next step
        return Err(anyhow!("Stdin hashing not yet implemented"));
    };

    // Output the hash
    if !matches!(verbosity, Verbosity::Quiet) {
        println!("Algorithm: {}", algorithm);
        if let Some(t) = text {
            println!("Text: {}", t);
        } else if let Some(f) = file {
            println!("File: {}", f);
        }
        println!("Digest: {}", hash);
    }

    Ok(())
}
