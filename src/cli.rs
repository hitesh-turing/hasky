use clap::{Parser, Subcommand};

/// A fast, flexible CLI for hashing with multiple algorithms
#[derive(Parser, Debug)]
#[command(name = "hashy")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
        #[arg(short, long, value_name = "TEXT", conflicts_with_all = &["file", "files"])]
        text: Option<String>,

        /// File to hash (use positional arguments for multiple files)
        #[arg(short, long, value_name = "FILE", conflicts_with = "files")]
        file: Option<String>,

        /// Files to hash in batch mode (positional arguments)
        #[arg(conflicts_with_all = &["text", "file"])]
        files: Vec<String>,

        /// Continue processing even if some files fail
        #[arg(long)]
        continue_on_error: bool,

        /// Output format [possible values: hex, base64, raw]
        #[arg(long, value_name = "FORMAT", conflicts_with = "json")]
        format: Option<String>,

        /// Use uppercase letters in hex output
        #[arg(long)]
        uppercase: bool,

        /// Output results as JSON
        #[arg(long, conflicts_with = "format")]
        json: bool,
    },

    /// Verify file integrity against checksum manifest
    Verify {
        /// Path to checksum manifest file
        checksums_file: String,

        /// Hash algorithm to use
        ///
        /// Supported algorithms: sha256 (default), sha512, blake3
        ///
        /// Insecure algorithms (sha1, md5) require --allow-insecure flag.
        #[arg(
            short,
            long,
            value_name = "ALGORITHM",
            default_value = "sha256",
            help = "Hash algorithm to use [possible values: sha256, sha512, blake3, sha1, md5]"
        )]
        algo: String,

        /// Allow use of insecure algorithms (SHA-1 and MD5)
        #[arg(long)]
        allow_insecure: bool,

        /// Manifest digest format [possible values: hex, base64, raw]
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,

        /// Continue verification even if some files fail
        #[arg(long)]
        continue_on_error: bool,
    },
}

/// Parameters returned from hash command
type HashParams<'a> = (
    &'a str,
    bool,
    Option<&'a str>,
    Option<&'a str>,
    &'a [String],
    bool,
    Option<&'a str>,
    bool,
    bool,
);

/// Parameters returned from verify command
type VerifyParams<'a> = (
    &'a str,         // algo
    bool,            // allow_insecure
    &'a str,         // checksums_file
    bool,            // continue_on_error
    Option<&'a str>, // format
);

impl Commands {
    pub fn get_hash_params(&self) -> Option<HashParams<'_>> {
        match self {
            Commands::Hash {
                algo,
                allow_insecure,
                text,
                file,
                files,
                continue_on_error,
                format,
                uppercase,
                json,
            } => Some((
                algo,
                *allow_insecure,
                text.as_deref(),
                file.as_deref(),
                files.as_slice(),
                *continue_on_error,
                format.as_deref(),
                *uppercase,
                *json,
            )),
            _ => None,
        }
    }

    pub fn get_verify_params(&self) -> Option<VerifyParams<'_>> {
        match self {
            Commands::Verify {
                checksums_file,
                algo,
                allow_insecure,
                format,
                continue_on_error,
            } => Some((
                algo,
                *allow_insecure,
                checksums_file.as_str(),
                *continue_on_error,
                format.as_deref(),
            )),
            _ => None,
        }
    }
}
