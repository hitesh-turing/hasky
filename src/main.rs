use anyhow::Result;
use clap::{Parser, Subcommand};

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

    // Placeholder for actual hashing logic (Step 3+)
    if !matches!(verbosity, Verbosity::Quiet) {
        println!("Hash command received - implementation pending");
        println!("Algorithm: {}", algo);
        if let Some(t) = text {
            println!("Text: {}", t);
        }
        if let Some(f) = file {
            println!("File: {}", f);
        }
    }

    Ok(())
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
}
