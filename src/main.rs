use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};

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

    // Get the input data to hash
    let data = if let Some(t) = text {
        t.as_bytes().to_vec()
    } else if let Some(_f) = file {
        // TODO: Implement file reading in next step
        return Err(anyhow!("File hashing not yet implemented"));
    } else {
        // TODO: Implement stdin reading in next step
        return Err(anyhow!("Stdin hashing not yet implemented"));
    };

    // Compute hash based on algorithm
    let hash = match algo.to_lowercase().as_str() {
        "sha256" => compute_sha256(&data),
        _ => return Err(anyhow!("Unsupported algorithm: {}", algo)),
    };

    // Output the hash
    if !matches!(verbosity, Verbosity::Quiet) {
        println!("Algorithm: {}", algo);
        if let Some(t) = text {
            println!("Text: {}", t);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_default_algo() {
        let cli = Cli::parse_from(&["hashy", "hash", "--text", "hello"]);
        match cli.command {
            Commands::Hash { algo, .. } => {
                assert_eq!(algo, "sha256");
            }
        }
    }

    #[test]
    fn test_custom_algo() {
        let cli = Cli::parse_from(&["hashy", "hash", "--algo", "blake3", "--text", "hello"]);
        match cli.command {
            Commands::Hash { algo, .. } => {
                assert_eq!(algo, "blake3");
            }
        }
    }

    #[test]
    fn test_verbose_flag() {
        let cli = Cli::parse_from(&["hashy", "--verbose", "hash", "--text", "hello"]);
        assert!(cli.verbose);
        assert!(!cli.quiet);
    }

    #[test]
    fn test_quiet_flag() {
        let cli = Cli::parse_from(&["hashy", "--quiet", "hash", "--text", "hello"]);
        assert!(cli.quiet);
        assert!(!cli.verbose);
    }

    #[test]
    fn test_sha256_empty_string() {
        let result = compute_sha256(b"");
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_sha256_abc() {
        let result = compute_sha256(b"abc");
        assert_eq!(
            result,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_sha256_hello() {
        let result = compute_sha256(b"hello");
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha256_longer_text() {
        let result = compute_sha256(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(
            result,
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
    }
}
