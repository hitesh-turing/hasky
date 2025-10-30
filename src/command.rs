use crate::algorithm::Algorithm;
use crate::hash::{hash_data, hash_file, hash_stdin};
use crate::verbosity::Verbosity;
use anyhow::{anyhow, Context, Result};
use atty::Stream;

/// Input source for hashing
enum InputSource {
    Text(String),
    File(String),
    Stdin,
}

/// Resolve the input source from CLI arguments
fn resolve_input_source(text: Option<&str>, file: Option<&str>) -> Result<InputSource> {
    // If text is provided, use it
    if let Some(t) = text {
        return Ok(InputSource::Text(t.to_string()));
    }

    // If file is provided, check if it's "-" (STDIN placeholder)
    if let Some(f) = file {
        if f == "-" {
            return Ok(InputSource::Stdin);
        }
        return Ok(InputSource::File(f.to_string()));
    }

    // No explicit input provided - check if STDIN is piped
    if atty::is(Stream::Stdin) {
        // Running interactively with no input - show friendly error
        return Err(anyhow!(
            "No input provided. Use --text, --file, or pipe data via STDIN."
        ));
    }

    // STDIN is piped (not a TTY), read from it
    Ok(InputSource::Stdin)
}

pub fn handle_hash(
    algo_str: &str,
    allow_insecure: bool,
    text: Option<&str>,
    file: Option<&str>,
    verbosity: Verbosity,
) -> Result<()> {
    // Parse algorithm
    let algorithm: Algorithm = algo_str.parse()?;

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

    // Resolve input source
    let input_source = resolve_input_source(text, file)?;

    // Store display info before consuming input_source
    let display_label = match &input_source {
        InputSource::Text(t) => Some(format!("Text: {}", t)),
        InputSource::File(f) => Some(format!("File: {}", f)),
        InputSource::Stdin => None, // Don't print label for STDIN
    };

    if matches!(verbosity, Verbosity::Verbose) {
        eprintln!("Using algorithm: {}", algorithm);
        match &input_source {
            InputSource::Text(t) => eprintln!("Hashing text: {}", t),
            InputSource::File(f) => eprintln!("Hashing file: {}", f),
            InputSource::Stdin => eprintln!("Hashing from STDIN"),
        }
    }

    // Compute hash based on algorithm and input type
    let hash = match input_source {
        InputSource::Text(t) => hash_data(algorithm, t.as_bytes()),
        InputSource::File(f) => {
            hash_file(algorithm, &f).with_context(|| format!("Failed to hash file: {}", f))?
        }
        InputSource::Stdin => hash_stdin(algorithm).context("Failed to hash STDIN")?,
    };

    // Output the hash
    if !matches!(verbosity, Verbosity::Quiet) {
        println!("Algorithm: {}", algorithm);
        if let Some(label) = display_label {
            println!("{}", label);
        }
        println!("Digest: {}", hash);
    }

    Ok(())
}
