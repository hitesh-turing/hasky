use crate::algorithm::Algorithm;
use crate::hash::{hash_data, hash_file, hash_stdin};
use crate::output::{HashJsonOutput, OutputFormat};
use crate::verbosity::Verbosity;
use anyhow::{anyhow, Context, Result};
use atty::Stream;
use serde_json;
use std::io::Write;

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

#[allow(clippy::too_many_arguments)]
pub fn handle_hash(
    algo_str: &str,
    allow_insecure: bool,
    text: Option<&str>,
    file: Option<&str>,
    format: Option<&str>,
    uppercase: bool,
    json: bool,
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

    // Store source name (will get size later for stdin)
    let source_name = match &input_source {
        InputSource::Text(_) => "text".to_string(),
        InputSource::File(_) => "file".to_string(),
        InputSource::Stdin => "stdin".to_string(),
    };

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

    // Compute hash based on algorithm and input type, also get input size
    let (hash_bytes, input_size) = match input_source {
        InputSource::Text(t) => {
            let size = t.len();
            (hash_data(algorithm, t.as_bytes()), size)
        }
        InputSource::File(f) => {
            // Hash first - this will give proper error if file doesn't exist
            let hash =
                hash_file(algorithm, &f).with_context(|| format!("Failed to hash file: {}", f))?;
            // Then get metadata for size (ignore errors, use 0 if we can't get it)
            let size = std::fs::metadata(&f).map(|m| m.len() as usize).unwrap_or(0);
            (hash, size)
        }
        InputSource::Stdin => hash_stdin(algorithm).context("Failed to hash STDIN")?,
    };

    // Determine output format
    let output_format = if json {
        None // JSON output doesn't use format enum
    } else if let Some(format_str) = format {
        match format_str.to_lowercase().as_str() {
            "hex" => Some(OutputFormat::Hex),
            "base64" => Some(OutputFormat::Base64),
            "raw" => Some(OutputFormat::Raw),
            _ => {
                return Err(anyhow!(
                    "Invalid format '{}'. Supported formats: hex, base64, raw",
                    format_str
                ));
            }
        }
    } else {
        None // Default multi-line format
    };

    // Output the hash
    if !matches!(verbosity, Verbosity::Quiet) {
        if json {
            // JSON output
            let digest_str = OutputFormat::Hex.format_bytes(&hash_bytes, false);
            let json_output = HashJsonOutput {
                algo: algorithm.name().to_string(),
                source: source_name,
                digest: digest_str,
                bytes: input_size,
            };
            let json_str = serde_json::to_string(&json_output)?;
            println!("{}", json_str);
        } else if let Some(fmt) = output_format {
            // Simplified single-line format output
            if fmt.is_raw() {
                // For raw format, write bytes directly to stdout
                std::io::stdout()
                    .write_all(&hash_bytes)
                    .context("Failed to write raw bytes to stdout")?;
            } else {
                let formatted = fmt.format_bytes(&hash_bytes, uppercase);
                println!("{}", formatted);
            }
        } else {
            // Default multi-line format
            println!("Algorithm: {}", algorithm);
            if let Some(label) = display_label {
                println!("{}", label);
            }
            println!(
                "Digest: {}",
                OutputFormat::Hex.format_bytes(&hash_bytes, false)
            );
        }
    }

    Ok(())
}
