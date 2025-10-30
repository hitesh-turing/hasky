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
        #[arg(short, long, value_name = "TEXT", conflicts_with = "file")]
        text: Option<String>,

        /// File to hash
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,

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
}

/// Parameters returned from hash command
type HashParams<'a> = (
    &'a str,
    bool,
    Option<&'a str>,
    Option<&'a str>,
    Option<&'a str>,
    bool,
    bool,
);

impl Commands {
    pub fn get_hash_params(&self) -> Option<HashParams<'_>> {
        match self {
            Commands::Hash {
                algo,
                allow_insecure,
                text,
                file,
                format,
                uppercase,
                json,
            } => Some((
                algo,
                *allow_insecure,
                text.as_deref(),
                file.as_deref(),
                format.as_deref(),
                *uppercase,
                *json,
            )),
        }
    }
}
