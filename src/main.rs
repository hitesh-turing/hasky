use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
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
        #[arg(short, long, value_name = "ALGORITHM", default_value = "sha256")]
        algo: String,

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
        Commands::Hash { algo, text, file } => {
            handle_hash(algo, text.as_deref(), file.as_deref(), verbosity)?;
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

fn handle_hash(
    algo: &str,
    text: Option<&str>,
    file: Option<&str>,
    verbosity: Verbosity,
) -> Result<()> {
    if matches!(verbosity, Verbosity::Verbose) {
        eprintln!("Using algorithm: {}", algo);
        if let Some(t) = text {
            eprintln!("Hashing text: {}", t);
        } else if let Some(f) = file {
            eprintln!("Hashing file: {}", f);
        } else {
            eprintln!("Ready to hash from stdin");
        }
    }

    // Compute hash based on algorithm and input type
    let hash = match algo.to_lowercase().as_str() {
        "sha256" => {
            if let Some(t) = text {
                compute_sha256(t.as_bytes())
            } else if let Some(f) = file {
                hash_file_sha256(f).with_context(|| format!("Failed to hash file: {}", f))?
            } else {
                // TODO: Implement stdin reading in next step
                return Err(anyhow!("Stdin hashing not yet implemented"));
            }
        }
        _ => return Err(anyhow!("Unsupported algorithm: {}", algo)),
    };

    // Output the hash
    if !matches!(verbosity, Verbosity::Quiet) {
        println!("Algorithm: {}", algo);
        if let Some(t) = text {
            println!("Text: {}", t);
        } else if let Some(f) = file {
            println!("File: {}", f);
        }
        println!("Output: {}", hash);
    }

    Ok(())
}

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Hash a file using SHA-256 by reading it in chunks (64 KiB).
/// This avoids loading the entire file into memory.
fn hash_file_sha256(file_path: &str) -> Result<String> {
    const CHUNK_SIZE: usize = 64 * 1024; // 64 KiB

    let file =
        File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .with_context(|| format!("Failed to read from file: {}", file_path))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}
