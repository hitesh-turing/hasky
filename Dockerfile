# Multi-stage build for a minimal final image
# Stage 1: Builder
FROM rust:latest as builder

WORKDIR /usr/src/hashy

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy source files to cache dependencies
# These need to match the module structure used by main.rs
RUN mkdir src && \
    echo "pub mod algorithm;" > src/lib.rs && \
    echo "pub mod cli;" >> src/lib.rs && \
    echo "pub mod command;" >> src/lib.rs && \
    echo "pub mod hash;" >> src/lib.rs && \
    echo "pub mod verbosity;" >> src/lib.rs && \
    echo "pub enum Algorithm {}" > src/algorithm.rs && \
    echo "#[derive(Debug)] pub enum Verbosity { Quiet, Normal, Verbose }" > src/verbosity.rs && \
    echo "pub struct Cli { pub quiet: bool, pub verbose: bool, pub command: Commands } #[derive(Debug)] pub enum Commands { Hash { algo: String, allow_insecure: bool, text: Option<String>, file: Option<String> } } impl Commands { pub fn get_hash_params(&self) -> Option<(&str, bool, Option<&str>, Option<&str>)> { None } } impl Cli { pub fn parse() -> Self { Self { quiet: false, verbose: false, command: Commands::Hash { algo: String::new(), allow_insecure: false, text: None, file: None } } } }" > src/cli.rs && \
    echo "use crate::algorithm::Algorithm; use crate::verbosity::Verbosity; pub fn handle_hash(_: &str, _: bool, _: Option<&str>, _: Option<&str>, _: Verbosity) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }" > src/command.rs && \
    echo "use crate::algorithm::Algorithm; pub fn hash_data(_: Algorithm, _: &[u8]) -> String { String::new() }" > src/hash.rs && \
    echo "use hashy::cli::Cli; use hashy::command::handle_hash; use hashy::verbosity::Verbosity; fn main() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY tests ./tests

# Build the application with optimizations
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/hashy/target/release/hashy /usr/local/bin/hashy

# Create a non-root user
RUN useradd -m -u 1000 hashy && \
    chown hashy:hashy /usr/local/bin/hashy

USER hashy
WORKDIR /home/hashy

ENTRYPOINT ["hashy"]
CMD ["--help"]
