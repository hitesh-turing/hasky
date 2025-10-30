# Hashy 🔐

A fast, flexible CLI for hashing with multiple algorithms built in Rust.

## Features

- 🚀 **Fast**: Built in Rust for maximum performance
- ⚡ **Parallel Processing**: Automatically parallelizes multi-file hashing using all available CPU cores
- 🔧 **Flexible**: Support for multiple hashing algorithms (SHA-256, SHA-512, BLAKE3, SHA-1, MD5)
- 🔒 **Security-Conscious**: Insecure algorithms (MD5, SHA-1) require explicit opt-in
- 📁 **File Hashing**: Efficiently hash files using chunked reading (64 KiB chunks) without loading entire files into memory
- 📦 **Batch Mode**: Hash multiple files in one command with automatic parallelization
- 🎨 **Output Formatting**: Multiple output formats (hex, base64, raw bytes) with JSON support for machine-readable output
- 🐳 **Dockerized**: Easy deployment with Docker
- ✅ **Tested**: Comprehensive unit and integration tests (60+ tests with verified test vectors)
- 📦 **Lightweight**: Minimal dependencies and small binary size

## Installation

### From Source

```bash
cargo install --path .
```

### Using Docker

```bash
docker build -t hashy .
docker run --rm hashy --help
```

## Usage

### Basic Commands

```bash
# Show help
hashy --help

# Show version
hashy --version

# Hash a text string (default: SHA-256)
hashy hash --text "hello world"

# Hash with different algorithms
hashy hash --algo sha256 --text "rust"
hashy hash --algo sha512 --text "rust"
hashy hash --algo blake3 --text "rust"

# Hash a file (efficiently processes large files in chunks)
hashy hash --file myfile.txt
hashy hash --algo blake3 --file ./Cargo.toml
hashy hash --algo sha512 --file myfile.txt

# Use insecure algorithms (requires --allow-insecure flag)
hashy hash --algo md5 --allow-insecure --file data.bin
hashy hash --algo sha1 --allow-insecure --text "legacy data"

# Hash matches standard tools like sha256sum
hashy hash --file myfile.txt  # Output matches: sha256sum myfile.txt

# Output formatting options
hashy hash --text "abc" --format hex           # Single-line hex output (default)
hashy hash --text "abc" --format hex --uppercase  # Uppercase hex
hashy hash --text "abc" --format base64       # Base64 encoded output
hashy hash --text "abc" --format raw          # Raw binary bytes
hashy hash --text "abc" --json                # JSON output for scripts/APIs

# Hash from STDIN
echo "hello" | hashy hash --algo sha256
echo "test" | hashy hash --algo sha256 --format hex
echo "data" | hashy hash --algo sha256 --json

# Batch mode - hash multiple files in parallel
hashy hash --algo sha256 file1.txt file2.txt file3.txt
hashy hash --algo sha256 --json *.txt
hashy hash --algo sha256 --continue-on-error file1.txt missing.txt file2.txt

# Verbose output
hashy --verbose hash --text "hello"

# Quiet mode (errors only)
hashy --quiet hash --text "hello"
```

### Global Flags

- `--verbose, -v`: Enable verbose output
- `--quiet, -q`: Suppress non-error output
- `--version`: Print version information
- `--help`: Print help information

### Hash Command

```bash
hashy hash [OPTIONS]

Options:
  -a, --algo <ALGORITHM>        Hash algorithm to use [default: sha256]
                                  Supported: sha256, sha512, blake3
                                  Insecure (requires --allow-insecure): sha1, md5
  -t, --text <TEXT>             Text to hash directly
  -f, --file <FILE>             File to hash (uses chunked reading for efficiency)
  <FILES>...                    Multiple files to hash in batch mode (automatically parallelized)
      --allow-insecure          Allow use of insecure algorithms (SHA-1 and MD5)
      --format <FORMAT>         Output format: hex, base64, or raw (simplified single-line output)
      --uppercase               Use uppercase letters in hex output (only with --format hex)
      --json                    Output results as JSON (conflicts with --format)
      --continue-on-error       Continue processing remaining files even if one fails (batch mode only)
  -h, --help                    Print help
```

**Supported Algorithms:**
- **SHA-256** (default): Secure, widely used hash algorithm
- **SHA-512**: Longer output (512 bits) for enhanced security
- **BLAKE3**: Fast, secure, modern hash function
- **SHA-1**: Insecure, requires `--allow-insecure` (⚠️ cryptographically broken)
- **MD5**: Insecure, requires `--allow-insecure` (⚠️ cryptographically broken)

**Security Note**: MD5 and SHA-1 are cryptographically broken and vulnerable to collision attacks. They are only available with the `--allow-insecure` flag for legacy compatibility or non-security purposes. A warning will be displayed when using these algorithms.

**Output Formats:**

By default, `hashy` outputs results in a multi-line format with labels:
```
Algorithm: sha256
Text: abc
Digest: ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
```

When using `--format`, output is simplified to a single line containing only the hash value, making it easier to parse in scripts:

- **`--format hex`**: Lowercase hexadecimal (default when using `--format`)
- **`--format hex --uppercase`**: Uppercase hexadecimal
- **`--format base64`**: Base64-encoded digest
- **`--format raw`**: Raw binary bytes written directly to stdout

**JSON Output:**

Use `--json` for machine-readable structured output suitable for APIs and pipelines:
```json
{
  "algo": "sha256",
  "source": "text",
  "digest": "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
  "bytes": 3
}
```

The JSON output includes:
- `algo`: The algorithm used
- `source`: Input source type (`text`, `file`, or `stdin`)
- `digest`: The hash digest in lowercase hex
- `bytes`: The size of the input in bytes

**Note**: File hashing uses chunked reading (64 KiB chunks) to efficiently process large files without loading them entirely into memory. The default output format matches standard tools like `sha256sum` for compatibility.

**Batch Mode**: When multiple files are provided as positional arguments, `hashy` automatically parallelizes the hashing process using all available CPU cores via the `rayon` crate. This significantly improves performance when hashing many files, especially on multi-core systems. The output order is preserved to match the input file order, regardless of parallel execution order.

## Development

### Prerequisites

- Rust 1.75+ (recommended)
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_hash_with_text
```

### Docker Development

```bash
# Build the Docker image
docker build -t hashy:dev .

# Run tests in Docker
docker run --rm hashy:dev hash --help

# Hash text using Docker
docker run --rm hashy:dev hash --text "hello world"

# Hash a file using Docker (mount volume)
docker run --rm -v $(pwd):/data hashy:dev hash --file /data/myfile.txt
```

## Roadmap

### ✅ Completed (Steps 1-8, 10)
- [x] Project scaffolding
- [x] CLI argument parsing with clap
- [x] Basic command structure
- [x] Global flags (--verbose, --quiet, --version)
- [x] Docker support
- [x] Unit and integration tests
- [x] SHA-256 implementation (Step 3)
- [x] File input with chunked streaming (64 KiB chunks) (Step 4)
- [x] Error handling with `anyhow`/`thiserror`
- [x] Hash verification against `sha256sum` for compatibility
- [x] Multiple algorithms (SHA-1, SHA-256, SHA-512, BLAKE3, MD5) (Step 5)
- [x] Security gating for insecure algorithms (--allow-insecure flag)
- [x] Comprehensive test vectors for all algorithms
- [x] STDIN support (Step 6)
- [x] Output formatting options (Step 7): hex, base64, raw, and JSON output formats
- [x] Batch mode with parallel file hashing (Step 8)
- [x] Parallel processing with rayon (Step 10) - automatically utilizes all CPU cores

### 🚧 Upcoming
- [ ] Verification mode (Step 9)
- [ ] Progress bars (Step 11)
- [ ] Config file support (Step 12)
- [ ] Security defaults (Step 13)
- [ ] HMAC support (Step 14)
- [ ] Directory & manifest generation (Step 15)
- [ ] Testing, fuzzing, and benches (Step 16)
- [ ] Packaging & distribution (Step 17)
- [ ] Documentation & examples (Step 18)
- [ ] Telemetry-free analytics & crash-proofing (Step 19)

## Testing Strategy

The project includes:
- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test CLI behavior end-to-end with 37+ comprehensive tests
- **Algorithm tests**: Test vectors verified for SHA-1, SHA-256, SHA-512, BLAKE3, and MD5
- **Compatibility tests**: Verify hash output matches standard tools (`sha256sum`)
- **Security tests**: Verify insecure algorithms are properly gated
- **Large file tests**: Verify chunked reading works correctly for files >64 KiB
- **Property tests**: (Coming in Step 16)
- **Fuzz tests**: (Coming in Step 16)
- **Benchmarks**: (Coming in Step 16)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [sha2](https://github.com/RustCrypto/hashes) - SHA-256 and SHA-512 hashing implementation
- [sha1](https://github.com/RustCrypto/hashes) - SHA-1 hashing implementation
- [md-5](https://github.com/RustCrypto/hashes) - MD5 hashing implementation
- [blake3](https://github.com/BLAKE3-team/BLAKE3) - BLAKE3 hashing implementation
- [digest](https://github.com/RustCrypto/hashes) - Unified trait for hash functions
- [hex](https://github.com/KokaKiwi/rust-hex) - Hexadecimal encoding
- [serde](https://github.com/serde-rs/serde) - Serialization framework for JSON output
- [serde_json](https://github.com/serde-rs/json) - JSON serialization
- [base64](https://github.com/marshallpierce/rust-base64) - Base64 encoding
- [rayon](https://github.com/rayon-rs/rayon) - Data parallelism library for parallel file processing
- [RustCrypto](https://github.com/RustCrypto) - Cryptographic algorithms

