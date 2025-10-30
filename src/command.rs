use crate::algorithm::Algorithm;
use crate::hash::{hash_data, hash_file, hash_stdin};
use crate::output::{BatchHashJsonOutput, HashJsonOutput, OutputFormat};
use crate::verbosity::Verbosity;
use anyhow::{anyhow, Context, Result};
use atty::Stream;
use rayon::prelude::*;
use serde_json;
use std::io::BufRead;
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
    files: &[String],
    continue_on_error: bool,
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

    // Handle batch mode (multiple files)
    if !files.is_empty() {
        return handle_batch_hash(
            algorithm,
            files,
            continue_on_error,
            output_format,
            uppercase,
            json,
            verbosity,
        );
    }

    // Resolve input source for single input
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
                file_path: None,
                success: None,
                error: None,
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

/// Result of hashing a single file in batch mode
#[derive(Debug)]
struct BatchHashResult {
    file_path: String,
    success: bool,
    hash_bytes: Option<Vec<u8>>,
    input_size: Option<usize>,
    error: Option<String>,
}

/// Handle batch hashing of multiple files
fn handle_batch_hash(
    algorithm: Algorithm,
    files: &[String],
    continue_on_error: bool,
    output_format: Option<OutputFormat>,
    uppercase: bool,
    json: bool,
    verbosity: Verbosity,
) -> Result<()> {
    if matches!(verbosity, Verbosity::Verbose) {
        eprintln!("Using algorithm: {}", algorithm);
        eprintln!("Hashing {} files", files.len());
    }

    // Process each file in parallel using rayon
    // Using par_iter() which preserves order when collected
    let results: Vec<BatchHashResult> = files
        .par_iter()
        .map(|file_path| {
            if matches!(verbosity, Verbosity::Verbose) {
                eprintln!("Hashing file: {}", file_path);
            }

            match hash_file(algorithm, file_path) {
                Ok(hash_bytes) => {
                    let input_size = std::fs::metadata(file_path)
                        .map(|m| m.len() as usize)
                        .unwrap_or(0);

                    BatchHashResult {
                        file_path: file_path.clone(),
                        success: true,
                        hash_bytes: Some(hash_bytes),
                        input_size: Some(input_size),
                        error: None,
                    }
                }
                Err(e) => {
                    let error_msg = format!("{}", e);
                    BatchHashResult {
                        file_path: file_path.clone(),
                        success: false,
                        hash_bytes: None,
                        input_size: None,
                        error: Some(error_msg),
                    }
                }
            }
        })
        .collect();

    // Collect errors and check if we should fail early
    let errors: Vec<(String, String)> = results
        .iter()
        .filter_map(|r| {
            if r.success {
                None
            } else {
                Some((r.file_path.clone(), r.error.clone().unwrap_or_default()))
            }
        })
        .collect();

    // If continue_on_error is false and we have errors, return early
    if !errors.is_empty() && !continue_on_error {
        let (file_path, error_msg) = &errors[0];
        return Err(anyhow!(
            "Failed to hash file '{}': {}",
            file_path,
            error_msg
        ));
    }

    // Output results
    if !matches!(verbosity, Verbosity::Quiet) {
        if json {
            // JSON output for batch mode
            let batch_output = BatchHashJsonOutput {
                algo: algorithm.name().to_string(),
                results: results
                    .iter()
                    .map(|r| HashJsonOutput {
                        algo: algorithm.name().to_string(),
                        source: "file".to_string(),
                        digest: if let Some(hash) = &r.hash_bytes {
                            OutputFormat::Hex.format_bytes(hash, false)
                        } else {
                            "ERROR".to_string()
                        },
                        bytes: r.input_size.unwrap_or(0),
                        file_path: Some(r.file_path.clone()),
                        success: Some(r.success),
                        error: r.error.clone(),
                    })
                    .collect(),
                summary: if !errors.is_empty() {
                    Some(format!(
                        "{} succeeded, {} failed",
                        results.iter().filter(|r| r.success).count(),
                        errors.len()
                    ))
                } else {
                    None
                },
            };
            let json_str = serde_json::to_string(&batch_output)?;
            println!("{}", json_str);
        } else {
            // Text output for batch mode
            for result in &results {
                if result.success {
                    if let Some(fmt) = output_format {
                        if fmt.is_raw() {
                            // For raw format, write bytes directly to stdout
                            if let Some(hash_bytes) = &result.hash_bytes {
                                std::io::stdout()
                                    .write_all(hash_bytes)
                                    .context("Failed to write raw bytes to stdout")?;
                            }
                        } else if let Some(hash_bytes) = &result.hash_bytes {
                            let formatted = fmt.format_bytes(hash_bytes, uppercase);
                            println!("{}  {}", result.file_path, formatted);
                        }
                    } else {
                        // Default format - one line per file
                        if let Some(hash_bytes) = &result.hash_bytes {
                            let digest = OutputFormat::Hex.format_bytes(hash_bytes, uppercase);
                            println!("{}  {}", result.file_path, digest);
                        }
                    }
                } else {
                    // Show error for failed files
                    if let Some(error) = &result.error {
                        println!("{}  ERROR: {}", result.file_path, error);
                    }
                }
            }

            // Show summary if there were errors
            if !errors.is_empty() {
                let success_count = results.iter().filter(|r| r.success).count();
                println!(
                    "Summary: {} succeeded, {} failed",
                    success_count,
                    errors.len()
                );
            }
        }
    }

    // Return error if any files failed and continue_on_error is false
    if !errors.is_empty() && !continue_on_error {
        return Err(anyhow!("One or more files failed to hash"));
    }

    Ok(())
}

/// Verify a checksum manifest (sha256sum-style) against the filesystem
pub fn handle_verify(
    algo_str: &str,
    allow_insecure: bool,
    checksums_file: &str,
    continue_on_error: bool,
    format: Option<&str>,
    verbosity: Verbosity,
) -> Result<()> {
    // Parse algorithm
    let algorithm: Algorithm = algo_str.parse()?;

    // Security gating
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
    if algorithm.is_insecure() && allow_insecure && !matches!(verbosity, Verbosity::Quiet) {
        eprintln!(
            "WARNING: {} is cryptographically broken and should not be used for security purposes.",
            algorithm.name()
        );
    }

    // Determine manifest digest format (default hex)
    let digest_format = if let Some(fmt) = format {
        match fmt.to_lowercase().as_str() {
            "hex" => OutputFormat::Hex,
            "base64" => OutputFormat::Base64,
            "raw" => OutputFormat::Raw,
            _ => {
                return Err(anyhow!(
                    "Invalid format '{}'. Supported formats: hex, base64, raw",
                    fmt
                ));
            }
        }
    } else {
        OutputFormat::Hex
    };

    // Read manifest
    let manifest_path = std::path::Path::new(checksums_file);
    let manifest_dir = manifest_path.parent().unwrap_or(std::path::Path::new("."));
    let file = std::fs::File::open(manifest_path)
        .with_context(|| format!("Failed to open checksums file: {}", checksums_file))?;
    let reader = std::io::BufReader::new(file);

    if matches!(verbosity, Verbosity::Verbose) {
        eprintln!("Using algorithm: {}", algorithm);
        eprintln!("Verifying manifest: {}", checksums_file);
    }

    let mut succeeded = 0usize;
    let mut failed = 0usize;

    for (idx, line_res) in reader.lines().enumerate() {
        let line = line_res.with_context(|| format!("Failed to read line {}", idx + 1))?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Parse line with two segments separated by at least one space (prefer two spaces)
        let (left, right_opt) = if let Some(pos) = trimmed.find("  ") {
            let (l, r) = trimmed.split_at(pos);
            (l.trim(), Some(r.trim_start_matches(' ').trim()))
        } else if let Some(pos) = trimmed.find(' ') {
            let (l, r) = trimmed.split_at(pos);
            (l.trim(), Some(r.trim_start()))
        } else {
            (trimmed, None)
        };

        // Helper: validate hex digest length for algorithm
        let is_valid_hex_for_algo = |s: &str| -> bool {
            let expected_len = match algorithm {
                Algorithm::Sha1 => 40,
                Algorithm::Sha256 => 64,
                Algorithm::Sha512 => 128,
                Algorithm::Blake3 => 64, // 32 bytes -> 64 hex chars
                Algorithm::Md5 => 32,
            };
            s.len() == expected_len && s.chars().all(|c| c.is_ascii_hexdigit())
        };

        let (expected_digest, path_str): (&str, &str) = match (right_opt, digest_format) {
            (Some(right), OutputFormat::Hex) => {
                // Try `<DIGEST>  <PATH>` first
                if is_valid_hex_for_algo(left) {
                    (left, right)
                } else if is_valid_hex_for_algo(right) {
                    // Accept `<PATH>  <DIGEST>`
                    (right, left)
                } else {
                    // Not a valid verification line (e.g., "Algorithm: sha256"), skip gracefully
                    if matches!(verbosity, Verbosity::Verbose) {
                        eprintln!("Skipping non-checksum line {}: {}", idx + 1, trimmed);
                    }
                    continue;
                }
            }
            (Some(right), _) => {
                // For non-hex formats, assume `<DIGEST>  <PATH>`
                (left, right)
            }
            (None, _) => {
                println!("{}: FAILED (invalid format)", trimmed);
                failed += 1;
                if !continue_on_error {
                    return Err(anyhow!("Invalid checksum line at {}", idx + 1));
                }
                continue;
            }
        };

        // Handle optional leading '*' in path (binary mode in coreutils)
        let path_clean = path_str.trim_start_matches('*');

        if matches!(verbosity, Verbosity::Verbose) {
            eprintln!("Verifying file: {}", path_clean);
        }

        // Resolve path relative to manifest
        let full_path = manifest_dir.join(path_clean);

        match hash_file(algorithm, full_path.to_str().unwrap_or(path_clean)) {
            Ok(actual_bytes) => {
                let actual_str = match digest_format {
                    OutputFormat::Hex => OutputFormat::Hex.format_bytes(&actual_bytes, false),
                    OutputFormat::Base64 => OutputFormat::Base64.format_bytes(&actual_bytes, false),
                    OutputFormat::Raw => {
                        // Raw expected must match exact bytes; compare using hex fallback (documented limitation)
                        OutputFormat::Hex.format_bytes(&actual_bytes, false)
                    }
                };

                if actual_str == expected_digest {
                    println!("{}: OK", path_clean);
                    succeeded += 1;
                } else {
                    println!(
                        "{}: FAILED (mismatch)\n  expected: {}\n  actual:   {}",
                        path_clean, expected_digest, actual_str
                    );
                    failed += 1;
                    if !continue_on_error {
                        return Err(anyhow!("Checksum mismatch for {}", path_clean));
                    }
                }
            }
            Err(e) => {
                println!("{}: FAILED ({})", path_clean, e);
                failed += 1;
                if !continue_on_error {
                    return Err(anyhow!("Failed to verify {}: {}", path_clean, e));
                }
            }
        }
    }

    if !matches!(verbosity, Verbosity::Quiet) {
        if failed == 0 {
            println!("All files verified successfully.");
        } else {
            println!("Summary: {} succeeded, {} failed", succeeded, failed);
        }
    }

    if failed > 0 {
        return Err(anyhow!("One or more files failed verification"));
    }

    Ok(())
}
