use crate::algorithm::Algorithm;
use crate::hash::{hash_data, hash_file};
use crate::verbosity::Verbosity;
use anyhow::{anyhow, Context, Result};

pub fn handle_hash(
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

